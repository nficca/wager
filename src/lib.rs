#![deny(clippy::unwrap_used)]
#![deny(unsafe_code)]
#![deny(missing_docs)]
#![warn(rust_2018_idioms)]

//! A library for dealing with Odds.

use std::num::NonZeroU32;

use derive_more::Display;
use fraction_simplification::simplify;

mod fraction_simplification;
mod rational_approximation;

const RATIONAL_APPROXIMATION_MAX_DENOMINATOR: i32 = 100;

/// An error that can occur when creating an Odd.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum OddError {
    /// The odd is invalid.
    InvalidOdd,
}

/// An odd.
#[derive(Debug, Clone, Copy, PartialEq, Display)]
pub enum Odd {
    /// A fractional odd.
    Fractional(Fractional),
    /// A decimal odd.
    Decimal(Decimal),
    /// A moneyline odd.
    Moneyline(Moneyline),
}

impl Odd {
    /// Parse an odd from a string.
    pub fn parse(input: &str) -> Result<Self, OddError> {
        let moneyline = Moneyline::parse(input);
        let decimal = Decimal::parse(input);
        let fractional = Fractional::parse(input);

        if let Ok(moneyline) = moneyline {
            Ok(Self::Moneyline(moneyline))
        } else if let Ok(decimal) = decimal {
            Ok(Self::Decimal(decimal))
        } else if let Ok(fractional) = fractional {
            Ok(Self::Fractional(fractional))
        } else {
            Err(OddError::InvalidOdd)
        }
    }
}

/// A fractional odd.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Display)]
#[display("{_0}/{_1}")]
pub struct Fractional(NonZeroU32, NonZeroU32);

/// A decimal odd.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Display)]
#[display("{_0}")]
pub struct Decimal(f64);

/// A moneyline odd.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Display)]
#[display("{}{}", if _0 > &0i64 { "+" } else { "-" }, _0.abs())]
pub struct Moneyline(i64);

impl Fractional {
    /// Create a new fractional odd.
    pub fn new(numerator: u32, denominator: u32) -> Result<Self, OddError> {
        let (numerator, denominator) = simplify(numerator, denominator);
        let numerator = numerator.try_into().map_err(|_| OddError::InvalidOdd)?;
        let denominator = denominator.try_into().map_err(|_| OddError::InvalidOdd)?;

        Ok(Self(numerator, denominator))
    }

    /// Parse a fractional odd from a string.
    pub fn parse(input: &str) -> Result<Self, OddError> {
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
        let (numerator, denominator) = rational_approximation::rational_approximation(
            value.0 - 1.0,
            RATIONAL_APPROXIMATION_MAX_DENOMINATOR,
        );

        if numerator <= 0 || denominator <= 0 {
            return Err(OddError::InvalidOdd);
        }

        Self::new(numerator.unsigned_abs(), denominator.unsigned_abs())
    }
}

impl TryFrom<Moneyline> for Fractional {
    type Error = OddError;

    fn try_from(value: Moneyline) -> Result<Self, Self::Error> {
        if value.0 > 0 {
            Fractional::new(value.0 as u32, 100)
        } else {
            Fractional::new(100, value.0.unsigned_abs() as u32)
        }
    }
}

impl Decimal {
    /// Create a new decimal odd.
    pub fn new(value: f64) -> Result<Self, OddError> {
        if value < 1.0 {
            return Err(OddError::InvalidOdd);
        }

        Ok(Self(value))
    }

    /// Parse a decimal odd from a string.
    pub fn parse(input: &str) -> Result<Self, OddError> {
        let value = input.trim().parse().map_err(|_| OddError::InvalidOdd)?;

        Self::new(value)
    }
}

impl TryFrom<Fractional> for Decimal {
    type Error = OddError;

    fn try_from(value: Fractional) -> Result<Self, Self::Error> {
        Self::new(1.0 + (value.0.get() as f64 / value.1.get() as f64))
    }
}

impl TryFrom<Moneyline> for Decimal {
    type Error = OddError;

    fn try_from(value: Moneyline) -> Result<Self, Self::Error> {
        if value.0 > 0 {
            Self::new((value.0 as f64 / 100.0) + 1.0)
        } else {
            Self::new((100.0 / value.0.abs() as f64) + 1.0)
        }
    }
}

impl Moneyline {
    /// Create a new moneyline odd.
    pub fn new(value: i64) -> Result<Self, OddError> {
        if value.abs() < 100 {
            return Err(OddError::InvalidOdd);
        }

        Ok(Self(value))
    }

    /// Parse a moneyline odd from a string.
    pub fn parse(input: &str) -> Result<Self, OddError> {
        let value = input.parse().map_err(|_| OddError::InvalidOdd)?;

        Self::new(value)
    }
}

impl TryFrom<Fractional> for Moneyline {
    type Error = OddError;

    fn try_from(value: Fractional) -> Result<Self, Self::Error> {
        let numerator = value.0.get() as f64;
        let denominator = value.1.get() as f64;

        let result = if numerator >= denominator {
            numerator / denominator * 100.0
        } else {
            -100.0 * denominator / numerator
        };

        Moneyline::new(result.round() as i64)
    }
}

impl TryFrom<Decimal> for Moneyline {
    type Error = OddError;

    fn try_from(value: Decimal) -> Result<Self, Self::Error> {
        let result = if value.0 >= 2.0 {
            (value.0 - 1.0) * 100.0
        } else {
            -100.0 / (value.0 - 1.0)
        };

        Moneyline::new(result.round() as i64)
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(Fractional::new(1, 2).unwrap(), Decimal::new(1.5).unwrap())]
    #[test_case(Fractional::new(2, 1).unwrap(), Decimal::new(3.0).unwrap())]
    #[test_case(Fractional::new(7, 9).unwrap(), Decimal::new(1.7777777777777777).unwrap())]
    fn fractional_to_decimal(value: Fractional, expected: Decimal) {
        assert_eq!(Decimal::try_from(value).unwrap(), expected);
        assert_eq!(TryInto::<Decimal>::try_into(value).unwrap(), expected);
    }

    #[test_case(Fractional::new(1, 2).unwrap(), Moneyline::new(-200).unwrap())]
    #[test_case(Fractional::new(2, 1).unwrap(), Moneyline::new(200).unwrap())]
    #[test_case(Fractional::new(7, 9).unwrap(), Moneyline::new(-129).unwrap())]
    fn fractional_to_moneyline(value: Fractional, expected: Moneyline) {
        assert_eq!(Moneyline::try_from(value).unwrap(), expected);
        assert_eq!(TryInto::<Moneyline>::try_into(value).unwrap(), expected);
    }

    #[test_case(Decimal::new(1.5).unwrap(), Fractional::new(1, 2).unwrap())]
    #[test_case(Decimal::new(3.0).unwrap(), Fractional::new(2, 1).unwrap())]
    #[test_case(Decimal::new(1.7777777777777777).unwrap(), Fractional::new(7, 9).unwrap())]
    fn decimal_to_fractional(value: Decimal, expected: Fractional) {
        assert_eq!(Fractional::try_from(value).unwrap(), expected);
        assert_eq!(TryInto::<Fractional>::try_into(value).unwrap(), expected);
    }

    #[test_case(Decimal::new(1.5).unwrap(), Moneyline::new(-200).unwrap())]
    #[test_case(Decimal::new(3.0).unwrap(), Moneyline::new(200).unwrap())]
    #[test_case(Decimal::new(1.7777777777777777).unwrap(), Moneyline::new(-129).unwrap())]
    fn decimal_to_moneyline(value: Decimal, expected: Moneyline) {
        assert_eq!(Moneyline::try_from(value).unwrap(), expected);
        assert_eq!(TryInto::<Moneyline>::try_into(value).unwrap(), expected);
    }

    #[test_case(Moneyline::new(-200).unwrap(), Fractional::new(1, 2).unwrap())]
    #[test_case(Moneyline::new(200).unwrap(), Fractional::new(2, 1).unwrap())]
    #[test_case(Moneyline::new(-128).unwrap(), Fractional::new(25, 32).unwrap())]
    fn moneyline_to_fractional(value: Moneyline, expected: Fractional) {
        assert_eq!(Fractional::try_from(value).unwrap(), expected);
        assert_eq!(TryInto::<Fractional>::try_into(value).unwrap(), expected);
    }

    #[test_case(Moneyline::new(-200).unwrap(), Decimal::new(1.5).unwrap())]
    #[test_case(Moneyline::new(200).unwrap(), Decimal::new(3.0).unwrap())]
    #[test_case(Moneyline::new(-129).unwrap(), Decimal::new(1.7751937984496124).unwrap())]
    fn moneyline_to_decimal(value: Moneyline, expected: Decimal) {
        assert_eq!(Decimal::try_from(value).unwrap(), expected);
        assert_eq!(TryInto::<Decimal>::try_into(value).unwrap(), expected);
    }

    #[test_case("1/2", Odd::Fractional(Fractional::new(1, 2).unwrap()))]
    #[test_case("2852 /  124", Odd::Fractional(Fractional::new(23, 1).unwrap()))]
    #[test_case("1.5", Odd::Decimal(Decimal::new(1.5).unwrap()))]
    #[test_case("1.7777777777777777", Odd::Decimal(Decimal::new(1.7777777777777777).unwrap()))]
    #[test_case("-200", Odd::Moneyline(Moneyline::new(-200).unwrap()))]
    #[test_case("+1200", Odd::Moneyline(Moneyline::new(1200).unwrap()))]
    fn parse(input: &str, expected: Odd) {
        assert_eq!(Odd::parse(input).unwrap(), expected);
    }

    #[test_case(Odd::Fractional(Fractional::new(1, 2).unwrap()), "1/2")]
    #[test_case(Odd::Fractional(Fractional::new(2852, 124).unwrap()), "23/1")]
    #[test_case(Odd::Decimal(Decimal::new(1.5).unwrap()), "1.5")]
    #[test_case(Odd::Decimal(Decimal::new(1.7777777777777777).unwrap()), "1.7777777777777777")]
    #[test_case(Odd::Moneyline(Moneyline::new(-200).unwrap()), "-200")]
    #[test_case(Odd::Moneyline(Moneyline::new(1200).unwrap()), "+1200")]
    fn display(value: Odd, expected: &str) {
        assert_eq!(format!("{}", value), expected);
    }
}
