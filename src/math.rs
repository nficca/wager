mod rational_approximation;

pub use rational_approximation::rational_approximation;

pub fn simplify_fraction(numerator: u32, denominator: u32) -> (u32, u32) {
    let gcd = gcd(numerator, denominator);
    let numerator = numerator / gcd;
    let denominator = denominator / gcd;

    (numerator, denominator)
}

fn gcd(a: u32, b: u32) -> u32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;
    #[test_case(1, 2, (1, 2))]
    #[test_case(10, 20, (1, 2))]
    #[test_case(46, 23, (2, 1))]
    fn simplify_test(numerator: u32, denominator: u32, expected: (u32, u32)) {
        let result = simplify_fraction(numerator, denominator);
        assert_eq!(result, expected);
    }
}
