#![allow(unused)]

use parking_lot::RwLock;
use scheesim_impl::*;
use scheesim_macro::{make_vec, vec_op};
use std::sync::atomic::{AtomicBool, AtomicUsize};
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

fn gauss_jacobi_iterative_solve(
    coeffs: &Vec<Vec<f64>>,
    rhs: &Vec<f64>,
    init_guess: &Vec<f64>,
    num_iter: usize,
    abs_tol: f64,
    rel_tol: f64,
) -> Vec<f64> {
    let coeffs_diag = coeffs.diag();
    let coeffs_diagflat = coeffs.diag_flat();
    let coeff_sub_diagflat: Vec<Vec<_>> = coeffs.sub(&coeffs_diagflat);

    let mut x = init_guess.clone();

    for _ in 0..num_iter {
        let x_old = x.clone();

        let coeffs_sub_diagflat_dot_x = coeff_sub_diagflat.dot(&x);
        let coeffs_sub_diagflat_dot_x_sub_rhs: Vec<f64> = coeffs_sub_diagflat_dot_x.sub(rhs);

        x = coeffs_sub_diagflat_dot_x_sub_rhs.div(&coeffs_diag);

        if x.is_convergent(&x_old, rel_tol, abs_tol) {
            return x;
        }
    }

    x
}

fn gauss_seidel_iterative_solve(
    coeffs: &Vec<Vec<f64>>,
    rhs: &Vec<f64>,
    init_guess: &Vec<f64>,
    num_iter: usize,
    abs_tol: f64,
    rel_tol: f64,
) -> Vec<f64> {
    let n = coeffs.len();
    let mut x = init_guess.clone();

    for _ in 0..num_iter {
        let mut x_curr = vec![0.0f64; n];

        for j in 0..n {
            let s1: f64 = coeffs[j][..j].to_vec().dot(&x_curr[..j].to_vec());
            let s2: f64 = coeffs[j][j + 1..].to_vec().dot(&x_curr[j + 1..].to_vec());

            let rhs_sub_s1s2 = rhs[j] - s1 - s2;

            x_curr[j] = rhs_sub_s1s2 / coeffs[j][j];
        }

        if x.is_convergent(&x_curr, rel_tol, abs_tol) {
            return x_curr;
        }

        x = x_curr;
    }

    x
}

pub enum LinearSystemSolve {
    LFactorize,
    GaussJacobi,
    GaussSeidel,
}

impl LinearSystemSolve {
    pub fn from_str(s: String) -> Self {
        match s.as_str() {
            "lud" | "lu_factorize" | "lu_factorise" => Self::LFactorize,
            "gj" | "gauss-jacobi" | "gauss_jacobi" | "jacobi" | "j" => Self::GaussJacobi,
            "gs" | "gauss-seidel" | "gauss_seidel" | "seidel" | "s" => Self::GaussSeidel,
            _ => panic!("Should be LU Factorize, Guauss-Jacobi or Gauss-Seidel"),
        }
    }

    pub fn solve(
        &self,
        coeffs: &Vec<Vec<f64>>,
        rhs: &Vec<f64>,
        init_guess: Option<&Vec<f64>>,
        num_iter: Option<usize>,
        abs_tol: Option<f64>,
        rel_tol: Option<f64>,
    ) -> Vec<f64> {
        match self {
            Self::LFactorize => {
                let solver = EliminatorSolver::new(coeffs, rhs);

                solver.factorize_eliminate_solve()
            }
            Self::GaussJacobi => gauss_jacobi_iterative_solve(
                coeffs,
                rhs,
                init_guess.unwrap(),
                num_iter.unwrap(),
                abs_tol.unwrap(),
                rel_tol.unwrap(),
            ),
            Self::GaussSeidel => gauss_seidel_iterative_solve(
                coeffs,
                rhs,
                init_guess.unwrap(),
                num_iter.unwrap(),
                abs_tol.unwrap(),
                rel_tol.unwrap(),
            ),
        }
    }
}

pub fn quasi_newton_iter_linear_until_converge<'a>(
    coeffs_linear: &Vec<Vec<f64>>,
    rhs: Vec<RhsValueType>,
    init_guess: &mut Vec<UnknownFactor>,
    alpha: f64,
    num_iter: usize,
    abs_tol: f64,
    rel_tol: f64,
    solver: LinearSystemSolve,
    num_iter_solver: Option<usize>,
    abs_tol_solver: Option<f64>,
    rel_tol_solver: Option<f64>,
) -> Vec<Vec<UnknownFactor>> {
    let n = coeffs_linear.len();
    let mut return_factors: Vec<Vec<UnknownFactor>> = vec![];
    return_factors.push(init_guess.to_vec());

    let mut last_unknowns = init_guess.clone();
    let mut last_rhs = rhs.clone();

    for k in 0..num_iter {
        let last_unknowns_values = last_unknowns.get_all();
        let rhs_at_k = last_rhs.calculate_linearize_at_kth(alpha, k);

        let solved_at_k = solver.solve(
            coeffs_linear,
            &rhs_at_k,
            Some(&last_unknowns_values),
            num_iter_solver,
            abs_tol_solver,
            rel_tol_solver,
        );

        return_factors.push(last_unknowns.clone());

        if solved_at_k.is_convergent(&last_unknowns_values, rel_tol, abs_tol) {
            return return_factors;
        }

        last_unknowns.set_all(solved_at_k);
        last_rhs.update_all(rhs_at_k);
    }

    return_factors
}
