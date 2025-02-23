//! Odds functionality and primitives.

use std::{
    fmt::{Debug, Display},
    str::FromStr,
};

mod decimal;
mod fractional;
mod moneyline;

pub use decimal::Decimal;
use derive_more::Display;
pub use fractional::Fractional;
pub use moneyline::Moneyline;

/// An error that can occur when creating an Odd.
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum OddError {
    /// The odd is invalid. Usually this is because it does not make sense,
    /// e.g. a fractional odd with a negative numerator or denominator.
    Invalid,

    /// An error that occurred when parsing an odd.
    ParseError,
}

/// Any representation of an odd. This is useful for handling odds generically.
#[derive(Debug, Clone, Copy, Display)]
pub enum AnyOdd {
    /// A fractional odd.
    Fractional(Fractional),
    /// A decimal odd.
    Decimal(Decimal),
    /// A moneyline odd.
    Moneyline(Moneyline),
}

impl Odd for AnyOdd {
    fn payout(&self, stake: f64) -> f64 {
        match self {
            AnyOdd::Decimal(decimal) => decimal.payout(stake),
            AnyOdd::Fractional(fractional) => fractional.payout(stake),
            AnyOdd::Moneyline(moneyline) => moneyline.payout(stake),
        }
    }
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

impl PartialEq for AnyOdd {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (AnyOdd::Decimal(a), AnyOdd::Decimal(b)) => a == b,
            (AnyOdd::Decimal(a), AnyOdd::Fractional(b)) => {
                Decimal::try_from(*b).map(|b| *a == b).unwrap_or(false)
            }
            (AnyOdd::Decimal(a), AnyOdd::Moneyline(b)) => {
                Decimal::try_from(*b).map(|b| *a == b).unwrap_or(false)
            }
            (AnyOdd::Fractional(a), AnyOdd::Fractional(b)) => a == b,
            (AnyOdd::Fractional(a), AnyOdd::Decimal(b)) => {
                Decimal::try_from(*a).map(|a| a == *b).unwrap_or(false)
            }
            (AnyOdd::Fractional(a), AnyOdd::Moneyline(b)) => {
                Fractional::try_from(*b).map(|b| *a == b).unwrap_or(false)
            }
            (AnyOdd::Moneyline(a), AnyOdd::Moneyline(b)) => a == b,
            (AnyOdd::Moneyline(a), AnyOdd::Decimal(b)) => {
                Decimal::try_from(*a).map(|a| a == *b).unwrap_or(false)
            }
            (AnyOdd::Moneyline(a), AnyOdd::Fractional(b)) => {
                Fractional::try_from(*a).map(|a| a == *b).unwrap_or(false)
            }
        }
    }
}

impl Eq for AnyOdd {}

impl PartialOrd for AnyOdd {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for AnyOdd {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (AnyOdd::Decimal(a), AnyOdd::Decimal(b)) => a.cmp(b),
            (AnyOdd::Decimal(a), AnyOdd::Fractional(b)) => Decimal::try_from(*b)
                .map(|b| a.cmp(&b))
                .unwrap_or(std::cmp::Ordering::Greater),
            (AnyOdd::Decimal(a), AnyOdd::Moneyline(b)) => Decimal::try_from(*b)
                .map(|b| a.cmp(&b))
                .unwrap_or(std::cmp::Ordering::Greater),
            (AnyOdd::Fractional(a), AnyOdd::Fractional(b)) => a.cmp(b),
            (AnyOdd::Fractional(a), AnyOdd::Decimal(b)) => Decimal::try_from(*a)
                .map(|a| a.cmp(b))
                .unwrap_or(std::cmp::Ordering::Less),
            (AnyOdd::Fractional(a), AnyOdd::Moneyline(b)) => Fractional::try_from(*b)
                .map(|b| a.cmp(&b))
                .unwrap_or(std::cmp::Ordering::Greater),
            (AnyOdd::Moneyline(a), AnyOdd::Moneyline(b)) => a.cmp(b),
            (AnyOdd::Moneyline(a), AnyOdd::Decimal(b)) => Decimal::try_from(*a)
                .map(|a| a.cmp(b))
                .unwrap_or(std::cmp::Ordering::Less),
            (AnyOdd::Moneyline(a), AnyOdd::Fractional(b)) => Fractional::try_from(*a)
                .map(|a| a.cmp(b))
                .unwrap_or(std::cmp::Ordering::Less),
        }
    }
}

/// An odd.
pub trait Odd:
    Debug
    + Display
    + Clone
    + Copy
    + PartialEq
    + Eq
    + PartialOrd
    + Ord
    + Into<AnyOdd>
    + FromStr<Err = OddError>
    + 'static
{
    /// Get the total (including the stake) payout for a given stake.
    ///
    /// Example
    /// ```rust
    /// use wager::odd::{Fractional, Odd};
    ///
    /// let fractional = Fractional::new(4, 1).unwrap();
    /// assert_eq!(fractional.payout(100.0), 500.0);
    /// ```
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

    #[test_case(&["2/1", "1/2"], &["1/2", "2/1"])]
    #[test_case(&["1/2", "2/1"], &["1/2", "2/1"])]
    #[test_case(&["-200", "+100"], &["-200", "+100"])]
    #[test_case(&["+100", "-200"], &["-200", "+100"])]
    #[test_case(&["1.2345", "1.5"], &["1.2345", "1.5"])]
    #[test_case(&["1.5", "1.2345"], &["1.2345", "1.5"])]
    #[test_case(&["1.2345", "2/1", "-800"], &["-800", "1.2345", "2/1"])]
    fn sort(values: &[&str], expected: &[&str]) {
        let values: Vec<AnyOdd> = values
            .iter()
            .map(|v| v.parse::<AnyOdd>().unwrap())
            .collect();
        let mut values = values.to_vec();
        values.sort();

        for (i, value) in values.iter().enumerate() {
            assert_eq!(value.to_string().as_str(), expected[i], "index: {}", i);
        }
    }
}
