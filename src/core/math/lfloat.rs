use rug::Float;
use std::ops::{Add, Sub, Mul, Div, Neg, Rem};
use std::str::FromStr;
use num_traits::{Zero, One, Signed, Num};
use rug::float::ParseFloatError;
use serde::{Serialize, Serializer};

pub const PRECISION: u32 = 1024;

#[derive(Clone)]
pub struct Lfloat(Float);

impl Lfloat {

    pub fn new() -> Self {
        Lfloat(Float::new(PRECISION))
    }

}

impl Zero for Lfloat {

    fn zero() -> Self {
        Lfloat(Float::with_val(PRECISION, 0))
    }

    fn is_zero(&self) -> bool {
        self.0 == 0
    }

}

impl One for Lfloat {

    fn one() -> Self {
        Lfloat(Float::with_val(PRECISION, 1))
    }

}

impl Signed for Lfloat {

    fn abs(&self) -> Self {
        Lfloat(self.0.clone().abs())
    }

    fn signum(&self) -> Self {
        Lfloat(self.0.clone().signum())
    }

    fn is_positive(&self) -> bool {
        self.0 > 0
    }

    fn is_negative(&self) -> bool {
        self.0 < 0
    }

    fn abs_sub(&self, other: &Self) -> Self {
        if self.0 > other.0 {
            Lfloat(Float::with_val(PRECISION, &self.0 - &other.0))
        } else {
            Lfloat(Float::with_val(PRECISION, 0))
        }
    }

}

impl Add for Lfloat {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Lfloat(self.0 + other.0)
    }
}

impl Sub for Lfloat {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Lfloat(self.0 - other.0)
    }
}

impl Mul for Lfloat {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Lfloat(self.0 * other.0)
    }
}

impl Div for Lfloat {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Lfloat(self.0 / other.0)
    }
}

impl Add for &Lfloat {
    type Output = Lfloat;

    fn add(self, other: Self) -> Lfloat {
        Lfloat(Float::with_val(PRECISION, &self.0 + &other.0))
    }
}

impl Sub for &Lfloat {
    type Output = Lfloat;

    fn sub(self, other: Self) -> Lfloat {
        Lfloat(Float::with_val(PRECISION, &self.0 - &other.0))
    }
}

impl Mul for &Lfloat {
    type Output = Lfloat;

    fn mul(self, other: Self) -> Lfloat {
        Lfloat(Float::with_val(PRECISION, &self.0 * &other.0))
    }
}

impl Div for &Lfloat {
    type Output = Lfloat;

    fn div(self, other: Self) -> Lfloat {
        Lfloat(Float::with_val(PRECISION, &self.0 / &other.0))
    }
}

impl FromStr for Lfloat {
    type Err = ParseFloatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Float::parse(s).map(|parsed| Lfloat(Float::with_val(PRECISION, parsed)))
    }
}

impl std::fmt::Debug for Lfloat {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl std::fmt::Display for Lfloat {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl PartialEq for Lfloat {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl PartialOrd for Lfloat {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl Neg for Lfloat {
    type Output = Lfloat;

    fn neg(self) -> Lfloat {
        Lfloat(-self.0)
    }
}

impl Neg for &Lfloat {
    type Output = Lfloat;

    fn neg(self) -> Lfloat {
        Lfloat(Float::with_val(PRECISION, -&self.0))
    }
}

impl Num for Lfloat {
    type FromStrRadixErr = ParseFloatError;

    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        Float::parse_radix(str, radix as i32)
            .map(|parsed| Lfloat(Float::with_val(PRECISION, parsed)))
    }
}

impl Rem for Lfloat {
    type Output = Lfloat;

    fn rem(self, other: Self) -> Lfloat {
        Lfloat(self.0.rem(&other.0))
    }
}

impl Rem for &Lfloat {
    type Output = Lfloat;

    fn rem(self, other: Self) -> Lfloat {
        Lfloat(Float::with_val(PRECISION, (&self.0).rem(&other.0)))
    }
}

impl Serialize for Lfloat {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0.to_string())
    }
}