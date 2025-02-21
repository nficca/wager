use derive_more::Display;

use super::{AnyOdd, Decimal, Fractional, Odd, OddConversion, OddError};

/// A moneyline odd.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Display)]
#[display("{}{}", if _0 > &0i64 { "+" } else { "-" }, _0.abs())]
pub struct Moneyline(i64);

impl Moneyline {
    /// Create a new moneyline odd.
    pub fn new(value: i64) -> Result<Self, OddError> {
        if value.abs() < 100 {
            return Err(OddError::InvalidOdd);
        }

        Ok(Self(value))
    }
}

impl From<Moneyline> for AnyOdd {
    fn from(value: Moneyline) -> Self {
        Self::Moneyline(value)
    }
}

impl Odd for Moneyline {
    /// Parse a moneyline odd from a string.
    fn parse(input: &str) -> Result<Self, OddError> {
        let value = input.parse().map_err(|_| OddError::InvalidOdd)?;

        Self::new(value)
    }
}

impl OddConversion<Fractional> for Moneyline {
    fn convert(&self) -> Result<Fractional, OddError> {
        if self.0 > 0 {
            Fractional::new(self.0 as u32, 100)
        } else {
            Fractional::new(100, self.0.unsigned_abs() as u32)
        }
    }
}

impl OddConversion<Decimal> for Moneyline {
    fn convert(&self) -> Result<Decimal, OddError> {
        if self.0 > 0 {
            Decimal::new((self.0 as f64 / 100.0) + 1.0)
        } else {
            Decimal::new((100.0 / self.0.abs() as f64) + 1.0)
        }
    }
}

impl OddConversion<Moneyline> for Moneyline {
    fn convert(&self) -> Result<Moneyline, OddError> {
        Ok(*self)
    }
}
