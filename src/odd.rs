//! Odds functionality and primitives.

use std::{
    fmt::{Debug, Display},
    str::FromStr,
};

mod decimal;
mod fractional;
mod moneyline;

pub use decimal::Decimal;
pub use fractional::Fractional;
pub use moneyline::Moneyline;

/// An error that can occur when creating an Odd.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum OddError {
    /// The odd is invalid.
    InvalidOdd,
}

/// Any representation of an odd. This is useful for handling odds generically.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum AnyOdd {
    /// A fractional odd.
    Fractional(Fractional),
    /// A decimal odd.
    Decimal(Decimal),
    /// A moneyline odd.
    Moneyline(Moneyline),
}

impl FromStr for AnyOdd {
    type Err = OddError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        input.parse::<Moneyline>().map(Into::into).or_else(|_| {
            input
                .parse::<Decimal>()
                .map(Into::into)
                .or_else(|_| input.parse::<Fractional>().map(Into::into))
        })
    }
}

/// An odd.
pub trait Odd:
    Debug
    + Display
    + Clone
    + Copy
    + PartialEq
    + PartialOrd
    + Into<AnyOdd>
    + FromStr<Err = OddError>
    + 'static
{
    /// Get the payout for a given stake.
    fn payout(&self, stake: f64) -> f64;
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

    #[test_case("1/2", AnyOdd::Fractional(Fractional::new(1, 2).unwrap()))]
    #[test_case("2852 /  124", AnyOdd::Fractional(Fractional::new(23, 1).unwrap()))]
    #[test_case("1.5", AnyOdd::Decimal(Decimal::new(1.5).unwrap()))]
    #[test_case("1.7777777777777777", AnyOdd::Decimal(Decimal::new(1.7777777777777777).unwrap()))]
    #[test_case("-200", AnyOdd::Moneyline(Moneyline::new(-200).unwrap()))]
    #[test_case("+1200", AnyOdd::Moneyline(Moneyline::new(1200).unwrap()))]
    fn parse(input: &str, expected: AnyOdd) {
        assert_eq!(input.parse::<AnyOdd>().unwrap(), expected);
    }

    #[test_case(Fractional::new(1, 2).unwrap(), "1/2")]
    #[test_case(Fractional::new(2852, 124).unwrap(), "23/1")]
    #[test_case(Decimal::new(1.5).unwrap(), "1.5")]
    #[test_case(Decimal::new(1.7777777777777777).unwrap(), "1.7777777777777777")]
    #[test_case(Moneyline::new(-200).unwrap(), "-200")]
    #[test_case(Moneyline::new(1200).unwrap(), "+1200")]
    fn display(value: impl Odd, expected: &str) {
        assert_eq!(format!("{}", value), expected);
    }
}
