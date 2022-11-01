use num_bigint::BigInt;
use std::ops::{Add, Mul, Sub};

/// Type implementing arbitrary-precision decimal arithmetic
#[derive(Debug)]
pub struct Decimal {
    number: BigInt,
    decimal_pow: BigInt,
}

impl Decimal {
    pub fn try_from(input: &str) -> Option<Decimal> {
        let parts: Vec<&str> = input.split(".").collect();
        Some(Self {
            number: BigInt::parse_bytes(parts.join("").as_bytes(), 10)?,
            decimal_pow: BigInt::from(10).pow(parts.get(1).unwrap_or(&"").len() as u32),
        })
    }
}

impl PartialEq for Decimal {
    fn eq(&self, other: &Self) -> bool {
        (self.number.clone() * other.decimal_pow.clone())
            == (other.number.clone() * self.decimal_pow.clone())
    }
}

impl PartialOrd for Decimal {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        (self.number.clone() * other.decimal_pow.clone())
            .partial_cmp(&(other.number.clone() * self.decimal_pow.clone()))
    }
}

impl Add for Decimal {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            number: (self.number * rhs.decimal_pow.clone())
                + (rhs.number * self.decimal_pow.clone()),
            decimal_pow: (self.decimal_pow * rhs.decimal_pow),
        }
    }
}

impl Sub for Decimal {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            number: (self.number * rhs.decimal_pow.clone())
                - (rhs.number * self.decimal_pow.clone()),
            decimal_pow: (self.decimal_pow * rhs.decimal_pow),
        }
    }
}

impl Mul for Decimal {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            number: self.number * rhs.number,
            decimal_pow: self.decimal_pow * rhs.decimal_pow,
        }
    }
}
