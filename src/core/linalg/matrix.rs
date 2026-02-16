use std::ops::{ Index, IndexMut, Add, Mul };
use std::cmp::min;
use num_traits::Zero;

pub struct Matrix<T> {
    pub n: usize,
    pub m: usize,
    values: Vec<Vec<T>>,
}

impl<T: Clone + Zero> Matrix<T> {

    pub fn new(n: usize, m: usize) -> Self {
        Matrix { n: n, m: m, values: vec![vec![T::zero(); m]; n] }
    }

    pub fn newDiagonal(n: usize, m: usize, diagonal: T) -> Self {
        let mut values = vec![vec![T::zero(); m]; n];
        for t in 0..min(n, m) {
            values[t][t] = diagonal.clone();
        }
        Matrix { n: n, m: m, values: values }
    }

    pub fn from(array: Vec<Vec<T>>) -> Self {
        Matrix {
            n: array.len(),
            m: if array.len() > 0 { array[0].len() } else { 0 },
            values: array.clone()
        }
    }

    pub fn clone(source: Matrix<T>) -> Self {
        Matrix {
            n: source.n,
            m: source.m,
            values: source.values.clone(),
        }
    }

}

impl<T> Index<(usize, usize)> for Matrix<T> {
    type Output = T;

    fn index(&self, (i, j): (usize, usize)) -> &T {
        &self.values[i][j]
    }
}

impl<T> IndexMut<(usize, usize)> for Matrix<T> {
    fn index_mut(&mut self, (i, j): (usize, usize)) -> &mut T {
        &mut self.values[i][j]
    }
}

impl<T: Add<Output = T> + Clone + Zero> Add for Matrix<T> {
    type Output = Matrix<T>;

    fn add(self, other: Matrix<T>) -> Matrix<T> {
        assert_eq!(self.n, other.n, "First dimesion mismatch: {} & {}", self.n, other.n);
        assert_eq!(self.m, other.m, "Second dimesion mismatch: {} & {}", self.m, other.m);

        let mut result: Matrix<T> = Matrix::new(self.n, self.m);
        for i in 0..self.n {
            for j in 0..other.m {
                result[(i, j)] = self[(i, j)].clone() + other[(i, j)].clone();
            }
        }
        
        result
    }
}

impl<T: Add<Output = T> + Mul<Output = T> + Clone + Zero> Mul for Matrix<T> {
    type Output = Matrix<T>;

    fn mul(self, other: Matrix<T>) -> Matrix<T> {
        assert_eq!(self.m, other.n, "Dimesions mismatch: {}x{} & {}x{}", self.n, self.m, other.n, other.m);

        let mut result: Matrix<T> = Matrix::new(self.n, other.m);
        for i in 0..self.n {
            for j in 0..other.m {
                for t in 0..self.m {
                    result[(i, j)] = result[(i, j)].clone() + self[(i, t)].clone() * other[(t, j)].clone();
                }
            }
        }

        result
    }
}