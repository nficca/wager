use derive_more::Display;

use crate::math;

use super::{AnyOdd, Fractional, Moneyline, Odd, OddConversion, OddError};

/// A decimal odd.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Display)]
#[display("{value}")]
pub struct Decimal {
    value: f64,
}

impl Decimal {
    /// Create a new decimal odd.
    pub fn new(value: f64) -> Result<Self, OddError> {
        if value < 1.0 {
            return Err(OddError::InvalidOdd);
        }

        Ok(Self { value })
    }

    /// Get the value of the decimal odd.
    pub fn value(&self) -> f64 {
        self.value
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

    /// Get the payout for a given stake.
    fn payout(&self, stake: f64) -> f64 {
        stake * self.value
    }
}

impl OddConversion<Decimal> for Decimal {
    fn convert(&self) -> Result<Decimal, OddError> {
        Ok(*self)
    }
}

impl OddConversion<Fractional> for Decimal {
    fn convert(&self) -> Result<Fractional, OddError> {
        let (numerator, denominator) = math::rational_approximation(self.value - 1.0);

        if numerator <= 0 || denominator <= 0 {
            return Err(OddError::InvalidOdd);
        }

        Fractional::new(numerator.unsigned_abs(), denominator.unsigned_abs())
    }
}

impl OddConversion<Moneyline> for Decimal {
    fn convert(&self) -> Result<Moneyline, OddError> {
        let result = if self.value >= 2.0 {
            (self.value - 1.0) * 100.0
        } else {
            -100.0 / (self.value - 1.0)
        };

        Moneyline::new(result.round() as i64)
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(1.5, 1.5)]
    #[test_case(3.0, 3.0)]
    #[test_case(1.7777777777777777, 1.7777777777777777)]
    fn valid(value: f64, expected: f64) {
        let decimal = Decimal::new(value).unwrap();
        assert_eq!(decimal.value(), expected);
    }

    #[test_case(0.5)]
    #[test_case(0.0)]
    #[test_case(-1.0)]
    fn invalid(value: f64) {
        let decimal = Decimal::new(value);
        assert!(decimal.is_err());
    }

    #[test_case(1.5, 100.0, 150.0)]
    #[test_case(3.0, 25.0, 75.0)]
    #[test_case(1.7777777777777777, 100.0, 177.77777777777777)]
    fn payout(value: f64, stake: f64, expected: f64) {
        let decimal = Decimal::new(value).unwrap();
        assert_eq!(decimal.payout(stake), expected);
    }
}
