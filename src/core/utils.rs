use crate::Matrix;

pub fn printMatrix<T: std::fmt::Display>(matrix: Matrix<T>) {
    for i in 0..matrix.n {
        for j in 0..matrix.m {
            print!("{} ", matrix[(i, j)])
        }
        print!("\n");
    }
}