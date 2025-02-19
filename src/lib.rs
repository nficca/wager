#![deny(clippy::unwrap_used)]
#![deny(unsafe_code)]
#![deny(missing_docs)]
#![warn(rust_2018_idioms)]

//! A library for dealing with Odds.

use std::num::NonZeroU32;

use fraction_simplification::simplify;

mod fraction_simplification;
mod rational_approximation;

const RATIONAL_APPROXIMATION_MAX_DENOMINATOR: i32 = 100;

/// A struct representing an Odd.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Odd {
    numerator: NonZeroU32,
    denominator: NonZeroU32,
}

/// An error that can occur when creating an Odd.
#[derive(Debug)]
pub enum OddError {
    /// The odd is invalid.
    InvalidOdd,
}

impl Odd {
    /// Create a new Odd from a fractional value.
    pub fn fractional<T: TryInto<NonZeroU32>>(
        numerator: T,
        denominator: T,
    ) -> Result<Self, OddError> {
        let numerator = numerator.try_into().map_err(|_| OddError::InvalidOdd)?;
        let denominator = denominator.try_into().map_err(|_| OddError::InvalidOdd)?;
        let (numerator, denominator) =
            simplify(numerator, denominator).map_err(|_| OddError::InvalidOdd)?;

        let numerator = NonZeroU32::new(numerator).ok_or(OddError::InvalidOdd)?;
        let denominator = NonZeroU32::new(denominator).ok_or(OddError::InvalidOdd)?;

        Ok(Self {
            numerator,
            denominator,
        })
    }

    /// Create a new Odd from a decimal value.
    pub fn decimal(value: f64) -> Result<Self, OddError> {
        let (numerator, denominator) = rational_approximation::rational_approximation(
            value - 1f64,
            RATIONAL_APPROXIMATION_MAX_DENOMINATOR,
        );

        let numerator = NonZeroU32::new(numerator as u32).ok_or(OddError::InvalidOdd)?;
        let denominator = NonZeroU32::new(denominator as u32).ok_or(OddError::InvalidOdd)?;

        Ok(Self {
            numerator,
            denominator,
        })
    }

    /// Create a new Odd from a moneyline value.
    pub fn moneyline(value: i64) -> Result<Self, OddError> {
        if value > 0 {
            Self::fractional(value.abs() as u32, 100)
        } else {
            Self::fractional(100, value.abs() as u32)
        }
    }

    /// Return the fractional representation of the Odd.
    pub fn as_fractional(&self) -> (u32, u32) {
        (self.numerator.get(), self.denominator.get())
    }

    /// Return decimal representation of the Odd.
    pub fn as_decimal(&self) -> f64 {
        let ratio = self.numerator.get() as f64 / self.denominator.get() as f64;
        ratio + 1f64
    }

    /// Return the moneyline representation of the Odd.
    pub fn as_moneyline(&self) -> i64 {
        let result = if self.numerator.get() >= self.denominator.get() {
            (self.numerator.get() as f64) / (self.denominator.get() as f64) * 100.0
        } else {
            -100.0 * (self.denominator.get() as f64) / (self.numerator.get() as f64)
        };

        result.round() as i64
    }
}

impl TryFrom<f64> for Odd {
    type Error = OddError;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        Self::decimal(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(1, 2, (1, 2))]
    #[test_case(7, 9, (7, 9))]
    #[test_case(46, 23, (2, 1))]
    fn fractional(numerator: u32, denominator: u32, expected: (u32, u32)) -> Result<(), OddError> {
        let odd = Odd::fractional(numerator, denominator)?;
        assert_eq!(odd.numerator.get(), expected.0);
        assert_eq!(odd.denominator.get(), expected.1);

        Ok(())
    }

    #[test_case(1.5, (1, 2))]
    #[test_case(1.7777777777777777, (7, 9))]
    fn decimal(value: f64, expected: (u32, u32)) -> Result<(), OddError> {
        let odd = Odd::decimal(value)?;
        assert_eq!(odd.numerator.get(), expected.0);
        assert_eq!(odd.denominator.get(), expected.1);

        Ok(())
    }

    #[test_case(1, 2, -200)]
    #[test_case(7, 9, -129)]
    #[test_case(46, 23, 200)]
    fn moneyline(numerator: u32, denominator: u32, expected: i64) -> Result<(), OddError> {
        let odd = Odd::fractional(numerator, denominator)?;
        assert_eq!(odd.as_moneyline(), expected);

        Ok(())
    }

    #[test_case(1.5, (1, 2))]
    #[test_case(1.7777777777777777, (7, 9))]
    fn try_from_f64(value: f64, expected: (u32, u32)) -> Result<(), OddError> {
        let odd = Odd::try_from(value)?;
        assert_eq!(odd.numerator.get(), expected.0);
        assert_eq!(odd.denominator.get(), expected.1);

        Ok(())
    }

    #[test_case(1, 2, (1, 2))]
    #[test_case(7, 9, (7, 9))]
    #[test_case(46, 23, (2, 1))]
    fn as_fractional(
        numerator: u32,
        denominator: u32,
        expected: (u32, u32),
    ) -> Result<(), OddError> {
        let odd = Odd::fractional(numerator, denominator)?;
        assert_eq!(odd.as_fractional(), expected);

        Ok(())
    }

    #[test_case(1, 2, 1.5)]
    #[test_case(7, 9, 1.7777777777777777)]
    fn as_decimal(numerator: u32, denominator: u32, expected: f64) -> Result<(), OddError> {
        let odd = Odd::fractional(numerator, denominator)?;
        assert_eq!(odd.as_decimal(), expected);

        Ok(())
    }
}
