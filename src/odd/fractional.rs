use std::{num::NonZeroU32, str::FromStr};

use derive_more::Display;

use crate::math;

use super::{AnyOdd, Decimal, Moneyline, Odd, OddError};

/// A fractional odd.
///
/// Comprised of a numerator and a denominator. The numerator represents the number of units
/// a bettor will profit given that the denominator represents the number of units staked.
///
/// E.g. 4/1 means that for every unit staked, the bettor will profit 4 units.
#[derive(Debug, Clone, Copy, Display)]
#[display("{numerator}/{denominator}")]
pub struct Fractional {
    numerator: NonZeroU32,
    denominator: NonZeroU32,
}

impl Fractional {
    /// Create a new fractional odd from a `numerator` and `denominator`.
    ///   - Will be simplified to its lowest terms.
    ///   - Will error if the fraction is negative or zero.
    ///
    /// Example
    /// ```rust
    /// use wager::odd::Fractional;
    ///
    /// let fractional = Fractional::new(8, 2).unwrap();
    /// assert_eq!(fractional.numerator(), 4);
    /// assert_eq!(fractional.denominator(), 1);
    ///
    /// let fractional = Fractional::new(1, 0);
    /// assert!(fractional.is_err());
    /// ```
    pub fn new(numerator: u32, denominator: u32) -> Result<Self, OddError> {
        let (numerator, denominator) = math::simplify_fraction(numerator, denominator);
        let numerator = numerator.try_into().map_err(|_| OddError::Invalid)?;
        let denominator = denominator.try_into().map_err(|_| OddError::Invalid)?;

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
            .ok_or(OddError::ParseError)?
            .trim()
            .parse()
            .map_err(|_| OddError::ParseError)?;
        let denominator = parts
            .next()
            .ok_or(OddError::ParseError)?
            .trim()
            .parse()
            .map_err(|_| OddError::ParseError)?;

        Self::new(numerator, denominator)
    }
}

impl PartialEq for Fractional {
    fn eq(&self, other: &Self) -> bool {
        let lcd = math::lcm(self.denominator(), other.denominator());
        let numerator_a = self.numerator() * (lcd / self.denominator());
        let numerator_b = other.numerator() * (lcd / other.denominator());

        numerator_a == numerator_b
    }
}

impl Eq for Fractional {}

impl PartialOrd for Fractional {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Fractional {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let lcd = math::lcm(self.denominator(), other.denominator());
        let numerator_a = self.numerator() * (lcd / self.denominator());
        let numerator_b = other.numerator() * (lcd / other.denominator());

        numerator_a.cmp(&numerator_b)
    }
}

impl TryFrom<Decimal> for Fractional {
    type Error = OddError;

    /// Convert a decimal odd to a fractional odd.
    ///
    /// <div class="warning">
    /// This conversion is not always exact.
    /// Converting a decimal value to a fraction requires rational approximation.
    /// This means that <b>the resulting fraction may not exactly equal the decimal value</b>,
    /// but it should be very close.
    /// </div>
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
    use std::cmp::Ordering;
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

    #[test_case((1, 2), (1, 2), true)]
    #[test_case((1, 2), (2, 4), true)]
    #[test_case((1, 2), (3, 6), true)]
    fn eq(a: (u32, u32), b: (u32, u32), expected: bool) {
        let a = Fractional::new(a.0, a.1).unwrap();
        let b = Fractional::new(b.0, b.1).unwrap();
        assert_eq!(a == b, expected);
    }

    #[test_case((1, 2), (1, 2), Ordering::Equal)]
    #[test_case((1, 2), (2, 4), Ordering::Equal)]
    #[test_case((1, 2), (3, 6), Ordering::Equal)]
    #[test_case((1, 2), (2, 3), Ordering::Less)]
    #[test_case((2, 3), (1, 2), Ordering::Greater)]
    fn cmp(a: (u32, u32), b: (u32, u32), expected: Ordering) {
        let a = Fractional::new(a.0, a.1).unwrap();
        let b = Fractional::new(b.0, b.1).unwrap();
        assert_eq!(a.cmp(&b), expected);
    }

    #[test_case((1, 2), 100.0, 150.0)]
    #[test_case((2, 1), 25.0, 75.0)]
    #[test_case((7, 9), 100.0, 177.77777777777777)]
    fn payout(value: (u32, u32), stake: f64, expected: f64) {
        let fractional = Fractional::new(value.0, value.1).unwrap();
        assert_eq!(fractional.payout(stake), expected);
    }
}
