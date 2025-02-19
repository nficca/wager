#![deny(clippy::unwrap_used)]
#![deny(unsafe_code)]
#![deny(missing_docs)]
#![warn(rust_2018_idioms)]

//! A library for dealing with Odds.

use std::num::NonZeroU32;

use bon::bon;

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

#[bon]
impl Odd {
    /// Create a new Odd from a fractional value.
    #[builder]
    pub fn fractional<T: TryInto<NonZeroU32>>(
        numerator: T,
        denominator: T,
    ) -> Result<Self, OddError> {
        let numerator = numerator.try_into().map_err(|_| OddError::InvalidOdd)?;
        let denominator = denominator.try_into().map_err(|_| OddError::InvalidOdd)?;
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
    /// Convert the Odd to a decimal.
    pub fn to_decimal(&self) -> f64 {
        let ratio = self.numerator.get() as f64 / self.denominator.get() as f64;
        ratio + 1f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fractional() -> Result<(), OddError> {
        let odd = Odd::fractional().numerator(1).denominator(2).call()?;
        assert_eq!(odd.numerator.get(), 1);
        assert_eq!(odd.denominator.get(), 2);

        Ok(())
    }

    #[test]
    fn decimal() -> Result<(), OddError> {
        let odd = Odd::decimal(1.5)?;
        assert_eq!(odd.numerator.get(), 1);
        assert_eq!(odd.denominator.get(), 2);

        Ok(())
    }

    #[test]
    fn to_decimal() -> Result<(), OddError> {
        let odd = Odd::fractional().numerator(1).denominator(2).call()?;
        assert_eq!(odd.to_decimal(), 1.5);

        Ok(())
    }
}
