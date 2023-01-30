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




pub type NonLinearDerivitiveFunc = &'static dyn Fn(f64) -> f64;

#[derive(Clone)]
pub enum RhsValueType {
    LinearValue(f64),
    LinearizerFunc(NonLinearDerivitiveFunc, f64),
}

impl<'a> RhsValueType {
    pub fn new_linear(f: f64) -> Self {
        Self::LinearValue(f)
    }

    pub fn new_linearizer(f: NonLinearDerivitiveFunc, arg: f64) -> Self {
        Self::LinearizerFunc(f, arg)
    }

    pub fn result(&self) -> f64 {
        match self {
            RhsValueType::LinearValue(f) => *f,
            RhsValueType::LinearizerFunc(f, arg) => f(arg.clone()),
        }
    }

    pub fn update_arg(&mut self, value: f64) {
        match self {
            RhsValueType::LinearValue(_) => *self = Self::LinearValue(value),
            RhsValueType::LinearizerFunc(f, _) => *self = Self::LinearizerFunc(*f, value),
        }
    }
}

pub trait NonLinearCalculateKth {
    fn calculate_linearize_at_kth(&self, alpha: f64, k: usize) -> Vec<f64>;
    fn update_all(&mut self, v: Vec<f64>);
}

impl<'a> NonLinearCalculateKth for Vec<RhsValueType> {
    fn calculate_linearize_at_kth(&self, alpha: f64, k: usize) -> Vec<f64> {
        self.into_iter()
            .map(|itm| itm.result().dampen_ln(alpha, k))
            .collect()
    }

    fn update_all(&mut self, v: Vec<f64>) {
        self.iter_mut().zip(v.iter()).map(|(itm, value)| itm.update_arg(*value)).collect()
    }
}

#[derive(Clone)]
pub enum UnknownFactor {
    Voltage(f64),
    Current(f64),
}

impl UnknownFactor {
    pub fn zero_factor(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "v" | "voltage" | "volt" => Self::Voltage(0.0),
            "i" | "amperage" | "current" | "ampers" | "ampere" | "amper" => Self::Current(0.0),
            _ => panic!("Should be volts or ampers"),
        }
    }

    pub fn set(&mut self, value: f64) {
        match self {
            UnknownFactor::Voltage(_) => *self = Self::Voltage(value),
            UnknownFactor::Current(_) => *self = Self::Current(value),
        }
    }

    pub fn get(&self) -> f64 {
        match self {
            UnknownFactor::Voltage(f) => *f,
            UnknownFactor::Current(f) => *f,
        }
    }
}

pub trait GetSetAll {
    fn get_all(&self) -> Vec<f64>;
    fn set_all(&mut self, rep: Vec<f64>);
}

impl GetSetAll for Vec<UnknownFactor> {
    fn get_all(&self) -> Vec<f64> {
        self.iter().map(|itm| itm.get()).collect()
    }

    fn set_all(&mut self, rep: Vec<f64>) {
        self.iter_mut()
            .zip(rep.iter().cloned())
            .for_each(|(uf, f)| uf.set(f));
    }
}

impl GetSetAll for Vec<f64> {
    fn get_all(&self) -> Vec<f64> {
        self.iter().cloned().map(|f| f).collect()
    }

    fn set_all(&mut self, rep: Vec<f64>) {
        self.iter_mut()
            .zip(rep.iter().cloned())
            .for_each(|(itm, rep)| *itm = rep);
    }
}
