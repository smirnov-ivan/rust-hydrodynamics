use std::ops::{ Index, IndexMut, Add, Mul };

pub struct Vector<T> {
    pub n: usize,
    values: Vec<T>,
}

impl<T: Clone + Default> Vector<T> {

    pub fn new(n: usize) -> Self {
        Vector { n: n, values: vec![T::default(); n] }
    }

    pub fn from(array: Vec<T>) -> Self {
        Vector { n: array.len(), values: array.clone() }
    }

    pub fn clone(source: Vector<T>) -> Self {
        Vector { n: source.n, values: source.values.clone() }
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

impl<T: Add<Output = T> + Clone + Default> Add for Vector<T> {
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

impl<T: Add<Output = T> + Mul<Output = T> + Clone + Default> Mul for Vector<T> {
    type Output = T;

    fn mul(self, other: Vector<T>) -> T {
        assert_eq!(self.n, other.n, "Dimesions mismatch: {} & {}", self.n, other.n);

        let mut result = T::default();
        for i in 0..self.n {
            result = result + self[i].clone() * other[i].clone();
        }

        result
    }
}