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
        vec_op!{ self * other accumulate }
    }
    fn add(&self, other: &Vec<f64>) -> f64 {
        vec_op!{ self + other accumulate }
    }
    fn sub(&self, other: &Vec<f64>) -> f64 {
        vec_op!{ self - other accumulate }
    }
    fn div(&self, other: &Vec<f64>) -> f64 {
        vec_op!{ self / other accumulate }        
    }
    fn rem(&self, other: &Vec<f64>) -> f64 {
        vec_op!{ self % other accumulate }        
    }
    fn mul(&self, other: &Vec<f64>) -> f64 {
        vec_op!{ self * other accumulate }
    }
}


impl VectorOps<f64, Vec<f64>> for Vec<f64> {
    fn dot(&self, _: &f64) -> Vec<f64> {
        panic!("Operation impossible! You can't get the dot product of a vector and a scalar!");
    }
    fn add(&self, other: &f64) -> Vec<f64> {
        vec_op!{ self + other scalar }
    }
    fn sub(&self, other: &f64) -> Vec<f64> {
        vec_op!{ self - other scalar }
    }
    fn div(&self, other: &f64) -> Vec<f64> {
        vec_op!{ self / other scalar }        
    }
    fn rem(&self, other: &f64) -> Vec<f64> {
        vec_op!{ self % other scalar }        
    }
    fn mul(&self, other: &f64) -> Vec<f64> {
        vec_op!{ self * other scalar }        
    }
}

impl VectorOps<Vec<Vec<f64>>, Vec<f64>> for Vec<Vec<f64>> {
    fn dot(&self, other: &Vec<Vec<f64>>) -> Vec<f64> {
        self
            .iter()
            .cloned()
            .zip(other.iter().cloned())
            .map(|(v1, v2)| vec_op!{ v1 * v2 accumulate } )
            .collect()
    }

    fn add(&self, other: &Vec<Vec<f64>>) -> Vec<f64> {
        self
            .iter()
            .cloned()
            .zip(other.iter().cloned())
            .map(|(v1, v2)| vec_op!{ v1 + v2 accumulate } )
            .collect()
    }

    fn sub(&self, other: &Vec<Vec<f64>>) -> Vec<f64> {
        self
            .iter()
            .cloned()
            .zip(other.iter().cloned())
            .map(|(v1, v2)| vec_op!{ v1 - v2 accumulate } )
            .collect()
    }

    fn div(&self, other: &Vec<Vec<f64>>) -> Vec<f64> {
        self
            .iter()
            .cloned()
            .zip(other.iter().cloned())
            .map(|(v1, v2)| vec_op!{ v1 / v2 accumulate } )
            .collect()
    }

    fn rem(&self, other: &Vec<Vec<f64>>) -> Vec<f64> {
        self
            .iter()
            .cloned()
            .zip(other.iter().cloned())
            .map(|(v1, v2)| vec_op!{ v1 % v2 accumulate } )
            .collect()
    }

    fn mul(&self, other: &Vec<Vec<f64>>) -> Vec<f64> {
        self
            .iter()
            .cloned()
            .zip(other.iter().cloned())
            .map(|(v1, v2)| vec_op!{ v1 * v2 accumulate } )
            .collect()
    }
}

impl VectorOps<Vec<f64>, Vec<f64>> for Vec<f64> {
    fn dot(&self, _: &Vec<f64>) -> Vec<f64> {
        panic!("Impossible operation! Dot product cannot return a vector!")
    }

    fn add(&self, other: &Vec<f64>) -> Vec<f64> {
        vec_op!{ self + other vector }
    }

    fn sub(&self, other: &Vec<f64>) -> Vec<f64> {
        vec_op!{ self - other vector }
    }

    fn div(&self, other: &Vec<f64>) -> Vec<f64> {
        vec_op!{ self / other vector }
    }

    fn rem(&self, other: &Vec<f64>) -> Vec<f64> {
        vec_op!{ self % other vector }
    }

    fn mul(&self, other: &Vec<f64>) -> Vec<f64> {
        vec_op!{ self * other vector }
    }
}

impl VectorOps<Vec<f64>, Vec<f64>> for Vec<Vec<f64>> {
    fn dot(&self, other: &Vec<f64>) -> Vec<f64> {
        self
        .iter()
        .cloned()
        .map(|v1| vec_op!{ v1 * other accumulate } )
        .collect()
    }

    fn add(&self, other: &Vec<f64>) -> Vec<f64> {
        self
        .iter()
        .cloned()
        .map(|v1| vec_op!{ v1 + other accumulate } )
        .collect()
    }

    fn sub(&self, other: &Vec<f64>) -> Vec<f64> {
        self
        .iter()
        .cloned()
        .map(|v1| vec_op!{ v1 - other accumulate } )
        .collect()
    }

    fn div(&self, other: &Vec<f64>) -> Vec<f64> {
        self
        .iter()
        .cloned()
        .map(|v1| vec_op!{ v1 / other accumulate } )
        .collect()
    }

    fn rem(&self, other: &Vec<f64>) -> Vec<f64> {
        self
        .iter()
        .cloned()
        .map(|v1| vec_op!{ v1 % other accumulate } )
        .collect()
    }

    fn mul(&self, other: &Vec<f64>) -> Vec<f64> {
        self
        .iter()
        .cloned()
        .map(|v1| vec_op!{ v1 * other accumulate } )
        .collect()
    }
}