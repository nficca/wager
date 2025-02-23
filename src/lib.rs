#![deny(clippy::unwrap_used)]
#![deny(unsafe_code)]
#![deny(missing_docs)]
#![warn(rust_2018_idioms)]

//! A library for dealing with Odds.
//!
//! Basic usage:
//!
//! ```rust
//! use wager::odd::{Decimal, Fractional, Moneyline};
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
//! use wager::odd::{Decimal, Fractional, Moneyline};
//!
//! let fractional = Fractional::new(1, 2).unwrap();
//! let decimal = Decimal::try_from(fractional).unwrap();
//! let moneyline = Moneyline::try_from(decimal).unwrap();
//! ```
//!
//! Parsing odds:
//!
//! ```rust
//! // Parse odds directly if you know the format ahead of time:
//! use wager::odd::{Decimal, Fractional, Moneyline, Odd, AnyOdd};
//!
//! let fractional = "1/2".parse::<Fractional>().unwrap();
//! let decimal = "1.5".parse::<Decimal>().unwrap();
//! let moneyline = "-200".parse::<Moneyline>().unwrap();
//!
//! // Parse odds generically:
//! match "1/2".parse::<AnyOdd>().unwrap() {
//!     AnyOdd::Fractional(fractional) => {} // Do something with fractional odd
//!     AnyOdd::Decimal(decimal) => {} // Do something with decimal odd
//!     AnyOdd::Moneyline(moneyline) => {} // Do something with moneyline odd
//! }
//! ```

mod math;
pub mod odd;
