use std::str::FromStr;

use derive_more::Display;

use super::{AnyOdd, Decimal, Fractional, Odd, OddError};

/// A moneyline odd.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Display)]
#[display("{}{}", if value > &0i64 { "+" } else { "-" }, value.abs())]
pub struct Moneyline {
    value: i64,
}

impl Moneyline {
    /// Create a new moneyline odd.
    pub fn new(value: i64) -> Result<Self, OddError> {
        if value.abs() < 100 {
            return Err(OddError::InvalidOdd);
        }

        Ok(Self { value })
    }

    /// Get the value of the moneyline odd.
    pub fn value(&self) -> i64 {
        self.value
    }
}

impl From<Moneyline> for AnyOdd {
    fn from(value: Moneyline) -> Self {
        Self::Moneyline(value)
    }
}

impl Odd for Moneyline {
    /// Get the payout for a given stake.
    fn payout(&self, stake: f64) -> f64 {
        if self.value > 0 {
            stake * (1.0 + self.value as f64 / 100.0)
        } else {
            stake * (1.0 + 100.0 / self.value.abs() as f64)
        }
    }
}

impl FromStr for Moneyline {
    type Err = OddError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let value = input.parse().map_err(|_| OddError::InvalidOdd)?;

        Self::new(value)
    }
}

impl TryFrom<Decimal> for Moneyline {
    type Error = OddError;

    fn try_from(value: Decimal) -> Result<Self, Self::Error> {
        let value = value.value();
        let result = if value >= 2.0 {
            (value - 1.0) * 100.0
        } else {
            -100.0 / (value - 1.0)
        };

        Self::new(result.round() as i64)
    }
}

impl TryFrom<Fractional> for Moneyline {
    type Error = OddError;

    fn try_from(value: Fractional) -> Result<Self, Self::Error> {
        let numerator = value.numerator() as f64;
        let denominator = value.denominator() as f64;

        let result = if numerator >= denominator {
            numerator / denominator * 100.0
        } else {
            -100.0 * denominator / numerator
        };

        Self::new(result.round() as i64)
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(100, 100)]
    #[test_case(-150, -150)]
    fn valid(value: i64, expected: i64) {
        let moneyline = Moneyline::new(value).unwrap();
        assert_eq!(moneyline.value(), expected);
    }

    #[test_case(99)]
    #[test_case(0)]
    #[test_case(-1)]
    fn invalid(value: i64) {
        let moneyline = Moneyline::new(value);
        assert!(moneyline.is_err());
    }

    #[test_case(100, 100.0, 200.0)]
    #[test_case(200, 25.0, 75.0)]
    #[test_case(-128, 100.0, 178.125)]
    fn payout(value: i64, stake: f64, expected: f64) {
        let moneyline = Moneyline::new(value).unwrap();
        assert_eq!(moneyline.payout(stake), expected);
    }
}
