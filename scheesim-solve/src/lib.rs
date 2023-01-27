#![allow(unused)]

use parking_lot::RwLock;
use scheesim_macro::{make_vec, vec_op};
use scheesim_vec::*;
use std::sync::atomic::{AtomicUsize, AtomicBool};
use std::thread;
use std::{
    ops::Deref,
    sync::{atomic::Ordering, Arc, Barrier},
};

macro_rules! copy_rwl_vec {
    ($vec:ident) => {{
        $vec.read()            
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

fn produce_list_of_intermittent_0s_and_1s(n: usize, odds: usize, evens: usize) -> Vec<usize> {
    (0..n)
        .into_iter()
        .map(|i| match i % 2 == 0 {
            true => evens,
            false => odds,
        })
        .collect()
}

trait IsZeroAt {
    fn is_zero_at(&self, i: usize, j: usize) -> bool;
}

impl IsZeroAt for Arc<RwLock<Vec<Vec<f64>>>> {
    fn is_zero_at(&self, i: usize, j: usize) -> bool {
        let self_cln = Arc::clone(&self);
        let is_zero = self_cln.read()[i][j] == 0.0;

        is_zero
    }
}

/// This function takes the barriers and select number of columns. Wait for all threads to reach
/// that row. Then factorizes and eliminates those columns.
fn barrier_rows_and_solve_cols(
    coeffs: Arc<RwLock<Vec<Vec<f64>>>>,
    lvals: Arc<RwLock<Vec<Vec<f64>>>>,
    permutation: Arc<RwLock<Vec<Vec<f64>>>>,
    row_barrier: Arc<Barrier>,
    phase_barrier: Arc<Barrier>,
    pivot_barrier: Arc<Barrier>,
    this_cols: Vec<usize>,
    size: AtomicUsize,
    thread_modulo: AtomicUsize,
) {
    let size_loaded = size.load(Ordering::Relaxed);
    let tn = thread_modulo.load(Ordering::Relaxed);
    (0..size_loaded).into_iter().for_each(|i| {
        row_barrier.wait();
        for k in (i..size_loaded) {
            let coeffs_clone = Arc::clone(&coeffs);
            let permutation_clone = Arc::clone(&permutation);
           
            match coeffs_clone.is_zero_at(i, i) {
                true => {
                    coeffs_clone.write().swap(i, k + 1);
                    permutation_clone.write().swap(i, k + 1);
                }
                false => break,
            }
        }
        
        pivot_barrier.wait();

        this_cols
            .iter()
            .cloned()
            .filter(|j| (*j > i) && (j % 2 == tn))
            .for_each(|j| {
                let ii = coeffs.read()[i][i];
                let ji = coeffs.read()[j][i];

                let factor = ji / ii;

                lvals
                    .write()
                    .get_mut(j)
                    .expect("Error getting jth outer lvalues")
                    .push_and_swap_remove(i, factor);

                let row_i_clone = coeffs.read()[i].clone();
                let row_j_clone = coeffs.read()[j].clone();

                let subtraction_prod_uj = row_j_clone.sub(&row_i_clone.mul(&factor));
                coeffs.write().push_and_swap_remove(j, subtraction_prod_uj);
            });
    });

    phase_barrier.wait();
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

    (0..size - 1).into_iter().rev().for_each(|i| {
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

pub struct EliminatorSolver {
    coeffs: Arc<RwLock<Vec<Vec<f64>>>>,
    lvals: Arc<RwLock<Vec<Vec<f64>>>>,
    permutation: Arc<RwLock<Vec<Vec<f64>>>>,
    row_barrier: Arc<Barrier>,
    phase_barrier: Arc<Barrier>,
    pivot_barrier: Arc<Barrier>,
    col_nums: Vec<Vec<usize>>,
    rhs: Vec<f64>,
    n: usize,
    num_threads: usize,
}

impl EliminatorSolver {
    pub fn new(coefficients: &Vec<Vec<f64>>, right_hand_side: &Vec<f64>) -> Self {
        let (n, m) = (coefficients.len(), coefficients[0].len());
        let num_threads = m - 1;

        let (lvals, permutation) = (
            Arc::new(RwLock::new(make_eye_matrix(n, m))),
            Arc::new(RwLock::new(make_eye_matrix(n, m))),
        );

        let coeffs = Arc::new(RwLock::new(coefficients.clone()));
        let rhs = right_hand_side.clone();

        let row_barrier = Arc::new(Barrier::new(num_threads));
        let phase_barrier = Arc::new(Barrier::new(num_threads));
        let pivot_barrier = Arc::new(Barrier::new(num_threads));

        let col_nums = vec![(1..n).into_iter().collect(); num_threads];

        Self {
            coeffs,
            lvals,
            permutation,
            row_barrier,
            phase_barrier,
            pivot_barrier,
            col_nums,
            rhs,
            n,
            num_threads,
        }
    }

    fn pivot_factor_eliminate_parallel_col(&self) {
        let (evens, odds) = (1, 0);
        let intermittent_vec =
            produce_list_of_intermittent_0s_and_1s(self.num_threads, odds, evens);

        let thrds = self
            .col_nums
            .iter()
            .cloned()
            .zip(intermittent_vec.into_iter())
            .map(|(this_cols, tn)| {
                let coeffs = Arc::clone(&self.coeffs);
                let lvals = Arc::clone(&self.lvals);
                let permutation = Arc::clone(&self.permutation);
                let row_barrier = Arc::clone(&self.row_barrier);
                let phase_barrier = Arc::clone(&self.phase_barrier);
                let pivot_barrier = Arc::clone(&self.pivot_barrier);
                let size = AtomicUsize::new(self.n);
                let thread_modulo = AtomicUsize::new(tn);

                thread::spawn(|| {
                    barrier_rows_and_solve_cols(
                        coeffs,
                        lvals,
                        permutation,
                        row_barrier,
                        phase_barrier,
                        pivot_barrier,
                        this_cols,
                        size,
                        thread_modulo,
                    )
                })
            })
            .collect::<Vec<_>>()
            .into_iter()
            .for_each(|t| t.join().expect("Error joining thread"));
    }

    fn forward_sub_inner_prod_perm_rhs(&self) -> Vec<f64> {
        let ref_lval = self.lvals.as_ref();
        let ref_perm = self.permutation.as_ref();

        let lvals_cpy = copy_rwl_vec!(ref_lval);
        let perm_cpy = copy_rwl_vec!(ref_perm);

        let dot_prod = perm_cpy.dot(&self.rhs);

        forward_substitute(&lvals_cpy, &dot_prod, self.n)
    }

    fn backward_sub_coeffs_fws_res(&self, forward_sub_res: &Vec<f64>) -> Vec<f64> {
        let ref_coeffs = self.coeffs.as_ref();
        let coeffs_cpy = copy_rwl_vec!(ref_coeffs);

        backward_substitution(&coeffs_cpy, forward_sub_res, self.n)
    }

    pub fn factorize_eliminate_solve(&self) -> Vec<f64> {
        self.pivot_factor_eliminate_parallel_col();
        let fws_res = self.forward_sub_inner_prod_perm_rhs();

        self.backward_sub_coeffs_fws_res(&fws_res)
    }
}
