#![deny(clippy::unwrap_used)]
#![deny(unsafe_code)]
#![deny(missing_docs)]
#![warn(rust_2018_idioms)]

//! A library for the representation and manipulation of betting odds.
//! Includes support for the following types of odds:
//!   - [`Fractional`](`odd::Fractional`)
//!   - [`Decimal`](`odd::Decimal`)
//!   - [`Moneyline`](`odd::Moneyline`)
//!
//! # Basic usage
//! ## Create
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
//! ## Parse
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
//!
//! ## Convert
//!
//! ```rust
//! use wager::odd::{Decimal, Fractional, Moneyline};
//!
//! let fractional = Fractional::new(1, 2).unwrap();
//! let decimal = Decimal::try_from(fractional).unwrap();
//! let moneyline = Moneyline::try_from(decimal).unwrap();
//! ```
//! <div class="warning">
//! It's very important to note that converting between odds is not always exact.
//! For example, converting a decimal odd to a fractional odd requires the
//! approximation of a real number from a rational number.
//! </div>
//!
//! ## Calculate payout
//!
//! ```rust
//! use wager::odd::{Decimal, Moneyline, Odd};
//!
//! let decimal = Decimal::new(1.5).unwrap();
//! let payout = decimal.payout(100.0);
//! assert_eq!(payout, 150.0);
//!
//! let moneyline = Moneyline::new(-200).unwrap();
//! let payout = moneyline.payout(100.0);
//! assert_eq!(payout, 150.0);
//! ```
//!
//! ## Compare
//!
//! ```rust
//! use wager::odd::{AnyOdd, Decimal, Moneyline};
//!
//! // With same type
//! let a = Decimal::new(1.5).unwrap();
//! let b = Decimal::new(1.6).unwrap();
//! assert!(a < b);
//!
//! // With different types
//! // Need to wrap in an AnyOdd first
//! let a = AnyOdd::Decimal(Decimal::new(1.4).unwrap());
//! let b = AnyOdd::Moneyline(Moneyline::new(-200).unwrap());
//! assert!(a < b);
//! ```

mod math;
pub mod odd;
