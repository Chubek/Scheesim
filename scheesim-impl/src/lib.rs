use num_traits::Num;
use std::cmp::PartialOrd;
use std::ops::Neg;

use scheesim_macro::vec_op;

pub trait PushAndSwapRemove<T> {
    fn push_and_swap_remove(&mut self, i: usize, val: T);
}

impl<T> PushAndSwapRemove<T> for Vec<T> {
    fn push_and_swap_remove(&mut self, i: usize, val: T) {
        self.push(val);
        self.swap_remove(i);
    }
}

pub trait VectorOps<T, U> {
    fn dot(&self, other: &T) -> U;
    fn add(&self, other: &T) -> U;
    fn sub(&self, other: &T) -> U;
    fn div(&self, other: &T) -> U;
    fn rem(&self, other: &T) -> U;
    fn mul(&self, other: &T) -> U;
}

impl VectorOps<Vec<f64>, f64> for Vec<f64> {
    fn dot(&self, other: &Vec<f64>) -> f64 {
        vec_op! { self * other accumulate }
    }
    fn add(&self, other: &Vec<f64>) -> f64 {
        vec_op! { self + other accumulate }
    }
    fn sub(&self, other: &Vec<f64>) -> f64 {
        vec_op! { self - other accumulate }
    }
    fn div(&self, other: &Vec<f64>) -> f64 {
        vec_op! { self / other accumulate }
    }
    fn rem(&self, other: &Vec<f64>) -> f64 {
        vec_op! { self % other accumulate }
    }
    fn mul(&self, other: &Vec<f64>) -> f64 {
        vec_op! { self * other accumulate }
    }
}

impl VectorOps<f64, Vec<f64>> for Vec<f64> {
    fn dot(&self, _: &f64) -> Vec<f64> {
        panic!("Operation impossible! You can't get the dot product of a vector and a scalar!");
    }
    fn add(&self, other: &f64) -> Vec<f64> {
        vec_op! { self + other scalar }
    }
    fn sub(&self, other: &f64) -> Vec<f64> {
        vec_op! { self - other scalar }
    }
    fn div(&self, other: &f64) -> Vec<f64> {
        vec_op! { self / other scalar }
    }
    fn rem(&self, other: &f64) -> Vec<f64> {
        vec_op! { self % other scalar }
    }
    fn mul(&self, other: &f64) -> Vec<f64> {
        vec_op! { self * other scalar }
    }
}

impl VectorOps<Vec<Vec<f64>>, Vec<f64>> for Vec<Vec<f64>> {
    fn dot(&self, other: &Vec<Vec<f64>>) -> Vec<f64> {
        self.iter()
            .cloned()
            .zip(other.iter().cloned())
            .map(|(v1, v2)| vec_op! { v1 * v2 accumulate })
            .collect()
    }

    fn add(&self, other: &Vec<Vec<f64>>) -> Vec<f64> {
        self.iter()
            .cloned()
            .zip(other.iter().cloned())
            .map(|(v1, v2)| vec_op! { v1 + v2 accumulate })
            .collect()
    }

    fn sub(&self, other: &Vec<Vec<f64>>) -> Vec<f64> {
        self.iter()
            .cloned()
            .zip(other.iter().cloned())
            .map(|(v1, v2)| vec_op! { v1 - v2 accumulate })
            .collect()
    }

    fn div(&self, other: &Vec<Vec<f64>>) -> Vec<f64> {
        self.iter()
            .cloned()
            .zip(other.iter().cloned())
            .map(|(v1, v2)| vec_op! { v1 / v2 accumulate })
            .collect()
    }

    fn rem(&self, other: &Vec<Vec<f64>>) -> Vec<f64> {
        self.iter()
            .cloned()
            .zip(other.iter().cloned())
            .map(|(v1, v2)| vec_op! { v1 % v2 accumulate })
            .collect()
    }

    fn mul(&self, other: &Vec<Vec<f64>>) -> Vec<f64> {
        self.iter()
            .cloned()
            .zip(other.iter().cloned())
            .map(|(v1, v2)| vec_op! { v1 * v2 accumulate })
            .collect()
    }
}

impl VectorOps<Vec<f64>, Vec<f64>> for Vec<f64> {
    fn dot(&self, _: &Vec<f64>) -> Vec<f64> {
        panic!("Impossible operation! Dot product cannot return a vector!")
    }

    fn add(&self, other: &Vec<f64>) -> Vec<f64> {
        vec_op! { self + other vector }
    }

    fn sub(&self, other: &Vec<f64>) -> Vec<f64> {
        vec_op! { self - other vector }
    }

    fn div(&self, other: &Vec<f64>) -> Vec<f64> {
        vec_op! { self / other vector }
    }

    fn rem(&self, other: &Vec<f64>) -> Vec<f64> {
        vec_op! { self % other vector }
    }

    fn mul(&self, other: &Vec<f64>) -> Vec<f64> {
        vec_op! { self * other vector }
    }
}

impl VectorOps<Vec<f64>, Vec<f64>> for Vec<Vec<f64>> {
    fn dot(&self, other: &Vec<f64>) -> Vec<f64> {
        self.iter()
            .cloned()
            .map(|v1| vec_op! { v1 * other accumulate })
            .collect()
    }

    fn add(&self, other: &Vec<f64>) -> Vec<f64> {
        self.iter()
            .cloned()
            .map(|v1| vec_op! { v1 + other accumulate })
            .collect()
    }

    fn sub(&self, other: &Vec<f64>) -> Vec<f64> {
        self.iter()
            .cloned()
            .map(|v1| vec_op! { v1 - other accumulate })
            .collect()
    }

    fn div(&self, other: &Vec<f64>) -> Vec<f64> {
        self.iter()
            .cloned()
            .map(|v1| vec_op! { v1 / other accumulate })
            .collect()
    }

    fn rem(&self, other: &Vec<f64>) -> Vec<f64> {
        self.iter()
            .cloned()
            .map(|v1| vec_op! { v1 % other accumulate })
            .collect()
    }

    fn mul(&self, other: &Vec<f64>) -> Vec<f64> {
        self.iter()
            .cloned()
            .map(|v1| vec_op! { v1 * other accumulate })
            .collect()
    }
}

impl VectorOps<Vec<Vec<f64>>, Vec<Vec<f64>>> for Vec<Vec<f64>> {
    fn dot(&self, _: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
        unimplemented!()
    }

    fn add(&self, other: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
        self.iter()
            .cloned()
            .zip(other.iter().cloned())
            .map(|(v1, v2)| vec_op! { v1 + v2 vector })
            .collect()
    }

    fn sub(&self, other: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
        self.iter()
            .cloned()
            .zip(other.iter().cloned())
            .map(|(v1, v2)| vec_op! { v1 - v2 vector })
            .collect()
    }

    fn div(&self, other: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
        self.iter()
            .cloned()
            .zip(other.iter().cloned())
            .map(|(v1, v2)| vec_op! { v1 / v2 vector })
            .collect()
    }

    fn rem(&self, other: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
        self.iter()
            .cloned()
            .zip(other.iter().cloned())
            .map(|(v1, v2)| vec_op! { v1 % v2 vector })
            .collect()
    }

    fn mul(&self, other: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
        self.iter()
            .cloned()
            .zip(other.iter().cloned())
            .map(|(v1, v2)| vec_op! { v1 * v2 vector })
            .collect()
    }
}

impl VectorOps<Vec<f64>, Vec<f64>> for f64 {
    fn dot(&self, _: &Vec<f64>) -> Vec<f64> {
        unimplemented!()
    }

    fn add(&self, other: &Vec<f64>) -> Vec<f64> {
        other.iter().cloned().map(|item| self + item).collect()
    }

    fn sub(&self, other: &Vec<f64>) -> Vec<f64> {
        other.iter().cloned().map(|item| self - item).collect()
    }

    fn div(&self, other: &Vec<f64>) -> Vec<f64> {
        other.iter().cloned().map(|item| self / item).collect()
    }

    fn rem(&self, other: &Vec<f64>) -> Vec<f64> {
        other.iter().cloned().map(|item| self % item).collect()
    }

    fn mul(&self, other: &Vec<f64>) -> Vec<f64> {
        other.iter().cloned().map(|item| self * item).collect()
    }
}

pub trait DiagFlat<T: Neg<Output = T> + Copy + PartialOrd> {
    fn diag(&self) -> Vec<T>;
    fn diag_flat(&self) -> Vec<Vec<T>>;
    fn is_diagonal(&self, epsilon: T) -> bool;
}

impl<T: Neg<Output = T> + Copy + PartialOrd> DiagFlat<T> for Vec<Vec<T>> {
    fn diag_flat(&self) -> Vec<Vec<T>> {
        assert!(self.len() == self[0].len(), "Must be square matrix!");

        let n = self.len();
        let mut diag_flat = Vec::<Vec<T>>::with_capacity(n);

        for i in 0..n {
            for j in 0..n {
                match i == j {
                    true => diag_flat[i][j] = self[i][j],
                    false => continue,
                }
            }
        }

        diag_flat
    }

    fn diag(&self) -> Vec<T> {
        assert!(self.len() == self[0].len(), "Must be square matrix!");

        let n = self.len();
        let mut diag_flat = Vec::<T>::with_capacity(n);

        for i in 0..n {
            for j in 0..n {
                match i == j {
                    true => diag_flat[i] = self[i][j],
                    false => continue,
                }
            }
        }

        diag_flat
    }

    fn is_diagonal(&self, epsilon: T) -> bool {
        let diagonal = self.diag();

        diagonal
            .into_iter()
            .all(|num| num > -epsilon && num < epsilon)
    }
}

pub trait ConvergentF64 {
    fn is_convergent(
        &self,
        other: &Vec<f64>,
        relative_tolerange: f64,
        absolute_tolerance: f64,
    ) -> bool;
}

impl ConvergentF64 for Vec<f64> {
    fn is_convergent(
        &self,
        other: &Vec<f64>,
        relative_tolerance: f64,
        absolute_tolerance: f64,
    ) -> bool {
        self.iter()
            .cloned()
            .zip(other.iter().cloned())
            .all(|(a, b)| (a - b).abs() <= (absolute_tolerance + relative_tolerance + b.abs()))
    }
}

pub trait Dampen<T: Num, U> {
    fn dampen_ln(&self, alpha: T, k: usize) -> U;
}

impl Dampen<f64, f64> for f64 {
    fn dampen_ln(&self, alpha: f64, k: usize) -> f64 {
        let alpha_div_k = alpha / k as f64;
        let sign_self = self.signum();
        let self_abs_mul_k_plus_one = 1. + ((k as f64) * self.abs());
        let ln_abs_mul_k_plus_one = self_abs_mul_k_plus_one.ln();

        alpha_div_k * sign_self * ln_abs_mul_k_plus_one
    }
}

impl Dampen<f64, Vec<f64>> for Vec<f64> {
    fn dampen_ln(&self, alpha: f64, k: usize) -> Vec<f64> {
        self.iter()
            .cloned()
            .map(|n| n.dampen_ln(alpha, k))
            .collect()
    }
}



#[derive(Clone)]
pub enum ElementStampType<'a> {
    Linear(f64),
    NonLinear(&'a dyn Fn(f64) -> f64),
    Dynamic(&'a dyn Fn(f64, u64) -> f64),
    MultiTerminalNonLinear(&'a dyn Fn(f64, f64) -> f64),
    MultiTerminalDyanmic(&'a dyn Fn(f64, f64, u64) -> f64),    
}

