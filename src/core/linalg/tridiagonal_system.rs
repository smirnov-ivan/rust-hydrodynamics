use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use crate::core::linalg::vector::Vector;
use std::ops::{Add, Sub, Mul, Div};
use num_traits::{Zero, One, Signed};
use std::fmt::{Debug, Display};
use std::cmp::{PartialEq, PartialOrd};

pub struct TridiagonalSystem<T> {
    n: usize,
    bound: (T, T),
    right_bound: (T, T),
    values: Vec<(T, T, T, T)>,
}

impl<T: Clone + Zero + PartialEq + PartialOrd +
    One + FromStr + Debug + Signed +
    Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> +
    Display
> TridiagonalSystem<T> {

    pub fn load(path: &String) -> Result<Self, &'static str> {
        let fileP = File::open(path);
        let mut values: Vec<(T, T, T, T)> = Vec::new();
        match fileP {
            Ok(file) => {
                let mut reader = BufReader::new(file);
                let mut params_str = String::new();
                reader.read_line(&mut params_str);
                let mut parts = params_str.split_whitespace();
                let kappa1 = parts.next()
                    .ok_or("No first bound")
                    .map_err(|_| "Parse error")?
                    .parse::<T>()
                    .map_err(|_| "Parse error")?;
                let kappa2 = parts.next()
                    .ok_or("No second bound")
                    .map_err(|_| "Parse error")?
                    .parse::<T>()
                    .map_err(|_| "Parse error")?;
                let mu1 = parts.next()
                    .ok_or("No first right bound")
                    .map_err(|_| "Parse error")?
                    .parse::<T>()
                    .map_err(|_| "Parse error")?;
                let mu2 = parts.next()
                    .ok_or("No second right bound")
                    .map_err(|_| "Parse error")?
                    .parse::<T>()
                    .map_err(|_| "Parse error")?;
                for line in reader.lines() {
                    let tmp = line.map_err(|_| "Parse error")?;
                    let mut parts = tmp.split_whitespace();
                        
                    let a = parts.next()
                        .ok_or("Not enouth values")
                        .map_err(|_| "Parse error")?
                        .parse::<T>()
                        .map_err(|_| "Parse error")?;
                    
                    let b = parts.next()
                        .ok_or("Not enouth values")
                        .map_err(|_| "Parse error")?
                        .parse::<T>()
                        .map_err(|_| "Parse error")?;
                    
                    let c = parts.next()
                        .ok_or("Not enouth values")
                        .map_err(|_| "Parse error")?
                        .parse::<T>()
                        .map_err(|_| "Parse error")?;
                    
                    let d = parts.next()
                        .ok_or("Not enouth values")
                        .map_err(|_| "Parse error")?
                        .parse::<T>()
                        .map_err(|_| "Parse error")?;

                    values.push((a, b, c, d));
                }
                Result::Ok(Self {
                    n: values.len() + 1,
                    bound: (kappa1, kappa2),
                    right_bound: (mu1, mu2),
                    values: values,
                })
            },
            Err(_) => Result::Err("Can't open file")
        }
    }

    pub fn solve(&self) -> Result<Vector<T>, &'static str> {
        let mut alpha: Vec<T> = Vec::new();
        let mut beta: Vec<T> = Vec::new();
        let mut result: Vec<T> = Vec::new();

        alpha.push(self.bound.0.clone());
        beta.push(self.right_bound.0.clone());

        for i in 0..self.values.len() {
            let alphaDenominator = self.values[i].1.clone() - alpha[i].clone() * self.values[i].0.clone();
            if alphaDenominator == T::zero() {
                return Err("Solution doesn't exist");
            }
            let nalpha = self.values[i].2.clone() / alphaDenominator; 
            alpha.push(nalpha);

            // инвертировал последнее, так как в наших обозачениях правая часть -fi
            let betaNumerator = beta[i].clone() * self.values[i].0.clone() - self.values[i].3.clone();
            let betaDenominator = self.values[i].1.clone() - alpha[i].clone() * self.values[i].0.clone();
            if betaDenominator == T::zero() {
                return Err("Solution doesn't exist");
            }
            let nbeta = betaNumerator / betaDenominator;
            beta.push(nbeta);
        }
        result.push((self.right_bound.1.clone() - beta[self.n - 2].clone() * self.bound.1.clone()) / (T::one() + alpha[self.n - 2].clone() * self.bound.1.clone()));
        
        for i in (0..alpha.len()).rev() {
            result.push(alpha[i].clone() * result[result.len() - 1].clone() + beta[i].clone());
        }

        result.reverse();
        Ok(Vector::from(result))
        //Ok(Vector::from(vec![T::zero(); self.n]))
    }

    pub fn checkT1(&self) -> bool {
        if self.bound.0.abs() > T::one() || self.bound.1.abs() >= T::one() {
            return false;
        };

        for i in 0..self.values.len() {
            if self.values[i].1.abs() < self.values[i].0.abs() + self.values[i].2.abs() {
                return false;
            }
        }

        true
    }

    pub fn checkT2(&self) -> bool {
        if self.bound.0.abs() > T::one() || self.bound.1.abs() > T::one() {
            return false;
        };

        for i in 0..self.values.len() {
            if self.values[i].0 == T::zero() || self.values[i].2 == T::zero() || self.values[i].1.abs() <= self.values[i].0.abs() + self.values[i].2.abs() {
                return false;
            }
        }

        true
    }

    pub fn getRight(&self) -> Vector<T> {
        let mut result: Vec<T> = Vec::new();
        result.push(self.right_bound.0.clone());
        for value in &self.values {
            result.push(value.3.clone());
        }
        result.push(self.right_bound.1.clone());
        Vector::from(result)
    }

}

impl<T:
    Zero + PartialOrd +
    Add<Output = T> + Sub<Output = T> + Mul<Output = T> +
    Display + Signed + Clone
> Mul<&Vector<T>> for &TridiagonalSystem<T> {
    type Output = Vector<T>;

    fn mul(self, other: &Vector<T>) -> Vector<T> {
        assert_eq!(self.n + 1, other.n, "Dimensions mismatch: {} + 1 & {}", self.n, other.n);

        let mut result = vec![T::zero(); self.n + 1];
        result[0] = other[0].clone() - self.bound.0.clone() * other[1].clone();
        for i in 1..=self.values.len() {
            result[i] = self.values[i - 1].0.clone() * other[i - 1].clone() - self.values[i - 1].1.clone() * other[i].clone() + self.values[i - 1].2.clone() * other[i + 1].clone();
        }
        result[self.n] = other[self.n].clone() - self.bound.1.clone() * other[self.n - 1].clone();

        Vector::from(result)
    }
}