//! This file is a Rust port of the C code here:
//! https://github.com/google/audio-to-tactile/blob/main/src/dsp/number_util.c:
//!
//! Copyright 2020 Google LLC
//!
//! Licensed under the Apache License, Version 2.0 (the "License");
//! you may not use this file except in compliance with the License.
//! You may obtain a copy of the License at
//!
//!     https://www.apache.org/licenses/LICENSE-2.0
//!
//! Unless required by applicable law or agreed to in writing, software
//! distributed under the License is distributed on an "AS IS" BASIS,
//! WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//! See the License for the specific language governing permissions and
//! limitations under the License.

const DEFAULT_MAX_TERMS: i32 = 47;
const DEFAULT_CONVERGENCE_TOLERANCE: f64 = 1e-9;
const DEFAULT_MAX_DENOMINATOR: i32 = 100;

pub fn rational_approximation(value: f64) -> (i32, i32) {
    approximate_rational(value, DEFAULT_MAX_DENOMINATOR)
}

fn approximate_rational(value: f64, max_denominator: i32) -> (i32, i32) {
    if max_denominator <= 0 {
        return (0, 0);
    } else if value > f64::MAX - 0.5 {
        return (i32::MAX, 1);
    } else if value < f64::MIN + 0.5 {
        return (i32::MIN, 1);
    }

    let sign: i32 = if value < 0.0 { -1 } else { 1 };

    let value = value.abs();

    if !value.is_finite() {
        return (0, 0);
    }

    let mut reciprocal_residual = value;
    let mut continued_fraction_term = value.floor() as i32;
    let mut prev_convergent = (1, 0);
    let mut convergent = (continued_fraction_term, 1);

    let mut n = 0;

    for term in 2.. {
        let next_residual = reciprocal_residual - (continued_fraction_term as f64);
        if next_residual.abs() <= DEFAULT_CONVERGENCE_TOLERANCE {
            return (sign * convergent.0, convergent.1);
        }

        reciprocal_residual = 1.0 / next_residual;
        continued_fraction_term = reciprocal_residual.floor() as i32;

        n = (max_denominator - prev_convergent.1) / convergent.1;
        if convergent.0 > 0 {
            let upper_bound = (i32::MAX - prev_convergent.0) / convergent.0;
            if n > upper_bound {
                n = upper_bound;
            }
        }

        if term >= DEFAULT_MAX_TERMS || continued_fraction_term >= n {
            break;
        }

        let next_convergent =
            append_continued_fraction_term(convergent, prev_convergent, continued_fraction_term);
        prev_convergent = convergent;
        convergent = next_convergent;
    }

    let mut best_approximation = convergent;
    let lower_bound = continued_fraction_term / 2;

    if n >= lower_bound {
        if n > continued_fraction_term {
            n = continued_fraction_term;
        }

        let semiconvergent = append_continued_fraction_term(convergent, prev_convergent, n);

        if (n > lower_bound)
            || (value - fraction_to_double(semiconvergent).abs()
                < (value - fraction_to_double(convergent)).abs())
        {
            best_approximation = semiconvergent;
        }
    }

    (sign * best_approximation.0, best_approximation.1)
}

fn append_continued_fraction_term(
    fraction: (i32, i32),
    prev_convergent: (i32, i32),
    term: i32,
) -> (i32, i32) {
    (
        term * fraction.0 + prev_convergent.0,
        term * fraction.1 + prev_convergent.1,
    )
}

fn fraction_to_double(fraction: (i32, i32)) -> f64 {
    (fraction.0 as f64) / (fraction.1 as f64)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(0.0, (0, 1))]
    #[test_case(1.0, (1, 1))]
    #[test_case(0.5, (1, 2))]
    #[test_case(0.3333333333333333, (1, 3))]
    #[test_case(0.14285714285714285, (1, 7))]
    #[test_case(0.125, (1, 8))]
    #[test_case(0.1111111111111111, (1, 9))]
    #[test_case(0.1, (1, 10))]
    #[test_case(0.01123595506, (1, 89))]
    #[test_case(0.5280898876, (47, 89))]
    fn test_rational_approximation(value: f64, expected: (i32, i32)) {
        assert_eq!(rational_approximation(value), expected);
    }
}
