use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use crate::Vector;
use std::ops::{Add, Sub, Mul, Div};
use num_traits::{One, Signed};
use std::fmt::{Debug, Display};
use std::cmp::{PartialEq, PartialOrd};

pub struct TridiogonalSystem<T> {
    n: usize,
    bound: (T, T),
    right_bound: (T, T),
    values: Vec<(T, T, T, T)>,
}

impl<T: Clone + Copy + Default + PartialEq + PartialOrd +
    One + FromStr + Debug + Signed +
    Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> +
    Display
> TridiogonalSystem<T> {

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
                    n: values.len() + 2,
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

        alpha.push(self.bound.0);
        beta.push(self.right_bound.0);

        for i in 0..self.values.len() {
            let alphaDenominator = self.values[i].1 - alpha[i] * self.values[i].0;
            if alphaDenominator == T::default() {
                return Err("Solution doesn't exist");
            }
            let nalpha = self.values[i].2 / alphaDenominator; 
            alpha.push(nalpha);

            // инвертировал последнее, так как в наших обозачениях правая часть -fi
            let betaNumerator = beta[i] * self.values[i].0 - self.values[i].3;
            let betaDenominator = self.values[i].1 - alpha[i] * self.values[i].0;
            if betaDenominator == T::default() {
                return Err("Solution doesn't exist");
            }
            let nbeta = betaNumerator / betaDenominator;
            beta.push(nbeta);
        }
        result.push((self.right_bound.1 - beta[self.n - 2] * self.bound.1) / (T::one() + alpha[self.n - 2] * self.bound.1));
        
        for i in (0..alpha.len()).rev() {
            result.push(alpha[i] * result[result.len() - 1] + beta[i]);
        }

        result.reverse();
        Ok(Vector::from(result))
        //Ok(Vector::from(vec![T::default(); self.n]))
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
            if self.values[i].0 == T::default() || self.values[i].2 == T::default() || self.values[i].1.abs() <= self.values[i].0.abs() + self.values[i].2.abs() {
                return false;
            }
        }

        true
    }

}