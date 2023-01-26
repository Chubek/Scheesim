#![allow(unused)]

use parking_lot::Mutex;
use scheesim_concurrent::ThreadPool;
use scheesim_macro::vec_op;
use scheesim_vec::*;
use std::sync::atomic::AtomicUsize;
use std::{
    ops::Deref,
    sync::{atomic::Ordering, Arc, Barrier},
};

macro_rules! copy_mutex_vec {
    ($vec:ident) => {{
        $vec.lock()
            .iter()
            .cloned()
            .map(|itm| itm)
            .collect::<Vec<_>>()
    }};
}

fn make_eye_matrix(n: usize, m: usize) -> Vec<Vec<f64>> {
    let mut eye = vec![vec![0.0f64; m]; n];

    for i_n in (0..n) {
        for i_m in (0..m) {
            if i_n == i_m {
                eye[i_n][i_m] = 1.0f64;
            }
        }
    }

    eye
}

/// This function takes the barriers and select number of columns. Wait for all threads to reach
/// that row. Then factorizes and eliminates those columns.
fn barrier_rows_and_solve_cols(
    coeffs: Arc<Mutex<Vec<Vec<f64>>>>,
    lvals: Arc<Mutex<Vec<Vec<f64>>>>,
    permutation: Arc<Mutex<Vec<Vec<f64>>>>,
    row_barrier: Arc<Barrier>,
    pivot_barrier: Arc<Barrier>,
    this_cols: Vec<usize>,
    size: AtomicUsize,
) {
    let barrier_clone = Arc::clone(&row_barrier);
    let size_loaded = size.load(Ordering::Relaxed);

    (0..size_loaded).into_iter().for_each(|i| {
        barrier_clone.wait();

        for k in (i + 1..size_loaded) {
            match coeffs
                .lock()
                .get(i)
                .expect("Error getting ith outher")
                .get(i)
                .expect("Error getting ith inner")
                .clone()
                == 0.0f64
            {
                true => {
                    pivot_barrier.wait();
                    coeffs.lock().swap(i, k + 1);
                    permutation.lock().swap(i, k + 1);
                }
                false => break,
            }
        }

        this_cols.iter().cloned().for_each(|j| {
            let factor = coeffs
                .lock()
                .get(j)
                .expect("Error getting jth outer or factorization operand")
                .get(i)
                .expect("Error getting ith inner for factorization operand")
                .clone()
                * coeffs
                    .lock()
                    .get(i)
                    .expect("Error getting ith outer for factorization RHS")
                    .get(i)
                    .expect("Error getting ith inner for factorization LHS")
                    .clone();
            lvals
                .lock()
                .get_mut(j)
                .expect("Error getting jth outer lvalues")
                .push_and_swap_remove(i, factor);

            let row_i_clone = coeffs
                .lock()
                .get(i)
                .expect("Error getting ith for row i elimination")
                .clone();
            let row_j_clone = coeffs
                .lock()
                .get(j)
                .expect("Error getting jth for row_j elimination")
                .clone();

            let subtraction_prod_uj = row_j_clone.sub(&row_i_clone.mul(&factor));
            coeffs.lock().push_and_swap_remove(j, subtraction_prod_uj);
        });
    });
}

fn forward_substitute(lvals: &Vec<Vec<f64>>, rhs: &Vec<f64>, size: usize) -> Vec<f64> {
    let mut y = vec![0.0f64; size];

    y.push_and_swap_remove(0, rhs[0] / lvals[0][0]);

    y.clone()
        .iter()
        .cloned()
        .enumerate()
        .for_each(|(i, y_prime)| {
            let y_range = y[..i].to_vec();
            let l_range = lvals
                .get(i)
                .expect("Error getting ith for lval in Forward Sub")[..i]
                .to_vec();

            let dot_product: f64 = l_range.dot(&y_range);
            y.push_and_swap_remove(i, (rhs[i] - dot_product) / lvals[i][i]);
        });

    y
}

fn backward_substitution(coeffs: &Vec<Vec<f64>>, rhs_dot: &Vec<f64>, size: usize) -> Vec<f64> {
    let mut x = vec![0.0f64; size];

    x.push_and_swap_remove(
        size - 1,
        rhs_dot
            .last()
            .expect("Error getting rhs last for Backward Sub")
            .clone()
            / coeffs[size - 1][size - 1],
    );

    (0..size - 2).into_iter().rev().for_each(|i| {
        let x_range = x[i..].to_vec();
        let coeff_range = coeffs
            .get(i)
            .expect("Error getting ith for coeffs in Backward Sub")[i..]
            .to_vec();

        let dot_product: f64 = x_range.dot(&coeff_range);

        x.push_and_swap_remove(i, (rhs_dot[i] - dot_product) / coeffs[i][i]);
    });

    x
}

struct EliminatorSolver {
    coeffs: Arc<Mutex<Vec<Vec<f64>>>>,
    lvals: Arc<Mutex<Vec<Vec<f64>>>>,
    permutation: Arc<Mutex<Vec<Vec<f64>>>>,
    row_barrier: Arc<Barrier>,
    pivot_barrier: Arc<Barrier>,
    col_nums: Vec<Vec<usize>>,
    rhs: Vec<f64>,
    n: usize,
    num_threads: usize,
}

impl EliminatorSolver {
    pub fn new(
        coefficients: &Vec<Vec<f64>>,
        right_hand_side: &Vec<f64>,
        num_threads: usize,
    ) -> Self {
        let (m, n) = (coefficients.len(), coefficients[0].len());
        let (lvals, permutation) = (
            Arc::new(Mutex::new(make_eye_matrix(n, m))),
            Arc::new(Mutex::new(make_eye_matrix(n, m))),
        );

        let coeffs = Arc::new(Mutex::new(coefficients.clone()));
        let rhs = right_hand_side.clone();

        let row_barrier = Arc::new(Barrier::new(num_threads));
        let pivot_barrier = Arc::new(Barrier::new(num_threads));

        let mut col_nums = vec![vec![0usize; 0]; num_threads];

        for i in (0..num_threads) {
            for j in (0..n) {
                if j % i == 0 {
                    col_nums[i].push(j);
                }
            }
        }

        Self {
            coeffs: coeffs,
            lvals,
            permutation,
            row_barrier,
            pivot_barrier,
            col_nums,
            rhs,
            n,
            num_threads,
        }
    }

    fn pivot_factor_eliminate_parallel_col(&self) {
        let pool = ThreadPool::new(self.num_threads);

        for this_cols in self.col_nums.iter().cloned() {
            let coeffs = Arc::clone(&self.coeffs);
            let lvals = Arc::clone(&self.lvals);
            let permutation = Arc::clone(&self.permutation);
            let row_barrier = Arc::clone(&self.row_barrier);
            let pivot_barrier = Arc::clone(&self.pivot_barrier);
            let size = AtomicUsize::new(self.n);

            pool.execute(|| {
                barrier_rows_and_solve_cols(
                    coeffs,
                    lvals,
                    permutation,
                    row_barrier,
                    pivot_barrier,
                    this_cols,
                    size,
                )
            });
        }
    }

    fn forward_sub_inner_prod_perm_rhs(&self) -> Vec<f64> {
        let ref_lval = self.lvals.as_ref();
        let ref_perm = self.permutation.as_ref();

        let lvals_cpy = copy_mutex_vec!(ref_lval);
        let perm_cpy = copy_mutex_vec!(ref_perm);

        let dot_prod = perm_cpy.dot(&self.rhs);

        forward_substitute(&lvals_cpy, &dot_prod, self.n)
    }

    fn backward_sub_coeffs_fws_res(&self, forward_sub_res: &Vec<f64>) -> Vec<f64> {
        let ref_coeffs = self.coeffs.as_ref();
        let coeffs_cpy = copy_mutex_vec!(ref_coeffs);

        backward_substitution(&coeffs_cpy, forward_sub_res, self.n)
    }

    pub fn factorize_eliminate_solve(&self) -> Vec<f64> {
        self.pivot_factor_eliminate_parallel_col();
        let fws_res = self.forward_sub_inner_prod_perm_rhs();

        self.backward_sub_coeffs_fws_res(&fws_res)
    }
}
