use std::str::FromStr;

use derive_more::Display;

use super::{AnyOdd, Fractional, Moneyline, Odd, OddError};

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
    /// Get the payout for a given stake.
    fn payout(&self, stake: f64) -> f64 {
        stake * self.value
    }
}

impl FromStr for Decimal {
    type Err = OddError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let value = input.trim().parse().map_err(|_| OddError::InvalidOdd)?;

        Self::new(value)
    }
}

impl TryFrom<Fractional> for Decimal {
    type Error = OddError;

    fn try_from(value: Fractional) -> Result<Self, Self::Error> {
        let numerator = value.numerator() as f64;
        let denominator = value.denominator() as f64;

        Self::new(1.0 + (numerator / denominator))
    }
}

impl TryFrom<Moneyline> for Decimal {
    type Error = OddError;

    fn try_from(value: Moneyline) -> Result<Self, Self::Error> {
        let value = value.value() as f64;

        if value > 0.0 {
            Self::new((value / 100.0) + 1.0)
        } else {
            Self::new((100.0 / value.abs()) + 1.0)
        }
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
