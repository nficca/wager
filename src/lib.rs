#![deny(clippy::unwrap_used)]
#![deny(unsafe_code)]
#![deny(missing_docs)]
#![warn(rust_2018_idioms)]

//! A library for dealing with Odds.
//!
//! Basic usage:
//!
//! ```rust
//! use odds_rs::odd::{Decimal, Fractional, Moneyline};
//!
//! // Fractional odds
//! let fractional = Fractional::new(1, 2).unwrap();
//!
//! // Decimal odds
//! let decimal = Decimal::new(1.5).unwrap();
//!
//! // Moneyline odds
//! let moneyline = Moneyline::new(-200).unwrap();
//! ```
//!
//! Converting between odds:
//!
//! ```rust
//! use odds_rs::odd::{Decimal, Fractional, Moneyline, OddConversion};
//!
//! let fractional = Fractional::new(1, 2).unwrap();
//! let decimal: Decimal = fractional.convert().unwrap();
//! let moneyline: Moneyline = fractional.convert().unwrap();
//! ```
//!
//! Parsing odds:
//!
//! ```rust
//! // Parse odds directly if you know the format ahead of time:
//! use odds_rs::odd::{Decimal, Fractional, Moneyline, Odd, AnyOdd};
//!
//! let fractional = Fractional::parse("1/2").unwrap();
//! let decimal = Decimal::parse("1.5").unwrap();
//! let moneyline = Moneyline::parse("-200").unwrap();
//!
//! // Parse odds generically:
//! match AnyOdd::parse("1/2").unwrap() {
//!     AnyOdd::Fractional(fractional) => {} // Do something with fractional odd
//!     AnyOdd::Decimal(decimal) => {} // Do something with decimal odd
//!     AnyOdd::Moneyline(moneyline) => {} // Do something with moneyline odd
//! }
//! ```

mod math;
pub mod odd;
