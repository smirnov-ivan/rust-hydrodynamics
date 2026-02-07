use crate::Matrix;
use crate::Vector;

pub fn printMatrix<T: std::fmt::Display>(matrix: Matrix<T>) {
    for i in 0..matrix.n {
        for j in 0..matrix.m {
            print!("{} ", matrix[(i, j)])
        }
        print!("\n");
    }
}

pub fn printVector<T: std::fmt::Display>(vector: Vector<T>) {
    for i in 0..vector.n {
        println!("{}", vector[i]);
    }
}