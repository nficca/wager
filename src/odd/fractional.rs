use std::num::NonZeroU32;

use derive_more::Display;

use crate::math;

use super::{AnyOdd, Decimal, Moneyline, Odd, OddConversion, OddError};

/// A fractional odd.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Display)]
#[display("{_0}/{_1}")]
pub struct Fractional(NonZeroU32, NonZeroU32);

impl Fractional {
    /// Create a new fractional odd.
    pub fn new(numerator: u32, denominator: u32) -> Result<Self, OddError> {
        let (numerator, denominator) = math::simplify_fraction(numerator, denominator);
        let numerator = numerator.try_into().map_err(|_| OddError::InvalidOdd)?;
        let denominator = denominator.try_into().map_err(|_| OddError::InvalidOdd)?;

        Ok(Self(numerator, denominator))
    }
}

impl From<Fractional> for AnyOdd {
    fn from(value: Fractional) -> Self {
        Self::Fractional(value)
    }
}

impl Odd for Fractional {
    /// Parse a fractional odd from a string.
    fn parse(input: &str) -> Result<Self, OddError> {
        let mut parts = input.split('/');
        let numerator = parts
            .next()
            .ok_or(OddError::InvalidOdd)?
            .trim()
            .parse()
            .map_err(|_| OddError::InvalidOdd)?;
        let denominator = parts
            .next()
            .ok_or(OddError::InvalidOdd)?
            .trim()
            .parse()
            .map_err(|_| OddError::InvalidOdd)?;

        Self::new(numerator, denominator)
    }
}

impl OddConversion<Fractional> for Fractional {
    fn convert(&self) -> Result<Fractional, OddError> {
        Ok(*self)
    }
}

impl OddConversion<Decimal> for Fractional {
    fn convert(&self) -> Result<Decimal, OddError> {
        Decimal::new(1.0 + (self.0.get() as f64 / self.1.get() as f64))
    }
}

impl OddConversion<Moneyline> for Fractional {
    fn convert(&self) -> Result<Moneyline, OddError> {
        let numerator = self.0.get() as f64;
        let denominator = self.1.get() as f64;

        let result = if numerator >= denominator {
            numerator / denominator * 100.0
        } else {
            -100.0 * denominator / numerator
        };

        Moneyline::new(result.round() as i64)
    }
}
