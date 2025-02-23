use std::{num::NonZeroU32, str::FromStr};

use derive_more::Display;

use crate::math;

use super::{AnyOdd, Decimal, Moneyline, Odd, OddError};

/// A fractional odd.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Display)]
#[display("{numerator}/{denominator}")]
pub struct Fractional {
    numerator: NonZeroU32,
    denominator: NonZeroU32,
}

impl Fractional {
    /// Create a new fractional odd.
    pub fn new(numerator: u32, denominator: u32) -> Result<Self, OddError> {
        let (numerator, denominator) = math::simplify_fraction(numerator, denominator);
        let numerator = numerator.try_into().map_err(|_| OddError::InvalidOdd)?;
        let denominator = denominator.try_into().map_err(|_| OddError::InvalidOdd)?;

        Ok(Self {
            numerator,
            denominator,
        })
    }

    /// Get the numerator of the fractional odd.
    pub fn numerator(&self) -> u32 {
        self.numerator.get()
    }

    /// Get the denominator of the fractional odd.
    pub fn denominator(&self) -> u32 {
        self.denominator.get()
    }
}

impl From<Fractional> for AnyOdd {
    fn from(value: Fractional) -> Self {
        Self::Fractional(value)
    }
}

impl Odd for Fractional {
    /// Get the payout for a given stake.
    fn payout(&self, stake: f64) -> f64 {
        stake * (1.0 + (self.numerator.get() as f64 / self.denominator.get() as f64))
    }
}

impl FromStr for Fractional {
    type Err = OddError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
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

impl TryFrom<Decimal> for Fractional {
    type Error = OddError;

    fn try_from(value: Decimal) -> Result<Self, Self::Error> {
        let value = value.value();
        let (numerator, denominator) = math::rational_approximation(value - 1.0);

        Self::new(numerator.unsigned_abs(), denominator.unsigned_abs())
    }
}

impl TryFrom<Moneyline> for Fractional {
    type Error = OddError;

    fn try_from(value: Moneyline) -> Result<Self, Self::Error> {
        if value.value() > 0 {
            Self::new(value.value() as u32, 100)
        } else {
            Self::new(100, value.value().unsigned_abs() as u32)
        }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case((1, 2), (1, 2))]
    #[test_case((2, 4), (1, 2))]
    #[test_case((124, 56), (31, 14))]
    fn valid(value: (u32, u32), expected: (u32, u32)) {
        let fractional = Fractional::new(value.0, value.1).unwrap();
        assert_eq!(fractional.numerator(), expected.0);
        assert_eq!(fractional.denominator(), expected.1);
    }

    #[test_case((0, 1))]
    #[test_case((1, 0))]
    #[test_case((0, 0))]
    fn invalid(value: (u32, u32)) {
        let fractional = Fractional::new(value.0, value.1);
        assert!(fractional.is_err());
    }

    #[test_case((1, 2), 100.0, 150.0)]
    #[test_case((2, 1), 25.0, 75.0)]
    #[test_case((7, 9), 100.0, 177.77777777777777)]
    fn payout(value: (u32, u32), stake: f64, expected: f64) {
        let fractional = Fractional::new(value.0, value.1).unwrap();
        assert_eq!(fractional.payout(stake), expected);
    }
}
