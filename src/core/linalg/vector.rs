use std::ops::{ Index, IndexMut, Add, Sub, Mul };
use std::cmp::{ PartialOrd };
use serde::{Serialize, Serializer};
use num_traits::{Signed, Zero};


//#[derive(Serialize)]
pub struct Vector<T> {
    //#[serde(skip)] 
    pub n: usize,
    values: Vec<T>,
}

impl<T: Clone + Zero + PartialOrd + Signed> Vector<T> {

    pub fn new(n: usize) -> Self {
        Vector { n: n, values: vec![T::zero(); n] }
    }

    pub fn from(array: Vec<T>) -> Self {
        Vector { n: array.len(), values: array.clone() }
    }

    pub fn clone(source: Vector<T>) -> Self {
        Vector { n: source.n, values: source.values.clone() }
    }

    pub fn norm(&self) -> T {
        let mut result = T::zero();
        for i in 0..self.n {
            if self.values[i].abs() > result {
                result = self.values[i].abs();
            }
        }
        result
    }

}

impl<T> Index<usize> for Vector<T> {
    type Output = T;

    fn index(&self, i: usize) -> &T {
        &self.values[i]
    }
}

impl<T> IndexMut<usize> for Vector<T> {
    fn index_mut(&mut self, i: usize) -> &mut T {
        &mut self.values[i]
    }
}

impl<T: Add<Output = T> + Clone + Zero + PartialOrd + Signed> Add for Vector<T> {
    type Output = Vector<T>;

    fn add(self, other: Vector<T>) -> Vector<T> {
        assert_eq!(self.n, other.n, "Dimesion mismatch: {} & {}", self.n, other.n);

        let mut result: Vector<T> = Vector::new(self.n);
        for i in 0..self.n {
            result[i] = self[i].clone() + other[i].clone();
        }
        result
    }
}

impl<T: Sub<Output = T> + Clone + Zero + PartialOrd + Signed> Sub for &Vector<T> {
    type Output = Vector<T>;

    fn sub(self, other: &Vector<T>) -> Vector<T> {
        assert_eq!(self.n, other.n, "Dimesion mismatch: {} & {}", self.n, other.n);

        let mut result: Vector<T> = Vector::new(self.n);
        for i in 0..self.n {
            result[i] = self[i].clone() - other[i].clone();
        }
        result
    }
}

impl<T: Add<Output = T> + Mul<Output = T> + Clone + Zero + PartialOrd + Signed> Mul for Vector<T> {
    type Output = T;

    fn mul(self, other: Vector<T>) -> T {
        assert_eq!(self.n, other.n, "Dimesions mismatch: {} & {}", self.n, other.n);

        let mut result = T::zero();
        for i in 0..self.n {
            result = result + self[i].clone() * other[i].clone();
        }

        result
    }
}

impl<T: Serialize> Serialize for Vector<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.values.serialize(serializer)
    }
}