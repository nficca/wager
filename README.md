# wager

A library for the representation and manipulation of betting odds. Supports the parsing, converting, comparing, and calculating the payouts of various types of odds.

## Basic usage

```rust
use wager::odd::{Decimal, Fractional, Moneyline, Odd, AnyOdd};

// Parse
let moneyline = "-200".parse::<Moneyline>().unwrap();

// Convert
let decimal = Decimal::try_from(moneyline).unwrap();

// Compare
let a = "+300".parse::<AnyOdd>().unwrap();
let b = "3/1".parse::<AnyOdd>().unwrap();
assert_eq!(a, b);

/// Calculate payout
assert_eq!(decimal.payout(200.0), 300.0);
```
