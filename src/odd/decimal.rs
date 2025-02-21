use derive_more::Display;

use crate::math;

use super::{AnyOdd, Fractional, Moneyline, Odd, OddConversion, OddError};

/// A decimal odd.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Display)]
#[display("{_0}")]
pub struct Decimal(f64);

impl Decimal {
    /// Create a new decimal odd.
    pub fn new(value: f64) -> Result<Self, OddError> {
        if value < 1.0 {
            return Err(OddError::InvalidOdd);
        }

        Ok(Self(value))
    }
}

impl From<Decimal> for AnyOdd {
    fn from(value: Decimal) -> Self {
        Self::Decimal(value)
    }
}

impl Odd for Decimal {
    /// Parse a decimal odd from a string.
    fn parse(input: &str) -> Result<Self, OddError> {
        let value = input.trim().parse().map_err(|_| OddError::InvalidOdd)?;

        Self::new(value)
    }
}

impl OddConversion<Decimal> for Decimal {
    fn convert(&self) -> Result<Decimal, OddError> {
        Ok(*self)
    }
}

impl OddConversion<Fractional> for Decimal {
    fn convert(&self) -> Result<Fractional, OddError> {
        let (numerator, denominator) = math::rational_approximation(self.0 - 1.0);

        if numerator <= 0 || denominator <= 0 {
            return Err(OddError::InvalidOdd);
        }

        Fractional::new(numerator.unsigned_abs(), denominator.unsigned_abs())
    }
}

impl OddConversion<Moneyline> for Decimal {
    fn convert(&self) -> Result<Moneyline, OddError> {
        let result = if self.0 >= 2.0 {
            (self.0 - 1.0) * 100.0
        } else {
            -100.0 / (self.0 - 1.0)
        };

        Moneyline::new(result.round() as i64)
    }
}
