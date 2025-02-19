#![deny(clippy::unwrap_used)]
#![deny(unsafe_code)]
#![deny(missing_docs)]
#![warn(rust_2018_idioms)]

//! A library for dealing with Odds.

use std::num::NonZeroU32;

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
    /// Create a new Odd.
    pub fn new<T: TryInto<NonZeroU32>>(numerator: T, denominator: T) -> Result<Self, OddError> {
        let numerator = numerator.try_into().map_err(|_| OddError::InvalidOdd)?;
        let denominator = denominator.try_into().map_err(|_| OddError::InvalidOdd)?;
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
    fn new() -> Result<(), OddError> {
        let odd = Odd::new(1, 2)?;
        assert_eq!(odd.numerator.get(), 1);
        assert_eq!(odd.denominator.get(), 2);

        Ok(())
    }

    #[test]
    fn to_decimal() -> Result<(), OddError> {
        let odd = Odd::new(1, 2)?;
        assert_eq!(odd.to_decimal(), 1.5);

        Ok(())
    }
}
