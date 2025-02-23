use std::str::FromStr;

use derive_more::Display;

use super::{AnyOdd, Decimal, Fractional, Odd, OddConversion, OddError};

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

impl OddConversion<Fractional> for Moneyline {
    fn convert(&self) -> Result<Fractional, OddError> {
        if self.value > 0 {
            Fractional::new(self.value as u32, 100)
        } else {
            Fractional::new(100, self.value.unsigned_abs() as u32)
        }
    }
}

impl OddConversion<Decimal> for Moneyline {
    fn convert(&self) -> Result<Decimal, OddError> {
        if self.value > 0 {
            Decimal::new((self.value as f64 / 100.0) + 1.0)
        } else {
            Decimal::new((100.0 / self.value.abs() as f64) + 1.0)
        }
    }
}

impl OddConversion<Moneyline> for Moneyline {
    fn convert(&self) -> Result<Moneyline, OddError> {
        Ok(*self)
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
