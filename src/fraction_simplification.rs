use std::num::NonZeroU32;

#[derive(Debug)]
pub enum SimplifyError {
    InvalidFraction,
}

pub fn simplify<T: TryInto<NonZeroU32>>(
    numerator: T,
    denominator: T,
) -> Result<(u32, u32), SimplifyError> {
    let numerator = numerator
        .try_into()
        .map_err(|_| SimplifyError::InvalidFraction)?;
    let denominator = denominator
        .try_into()
        .map_err(|_| SimplifyError::InvalidFraction)?;

    let gcd = gcd(numerator.get(), denominator.get());
    let numerator = numerator.get() / gcd;
    let denominator = denominator.get() / gcd;

    Ok((numerator, denominator))
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
        let result = simplify(numerator, denominator).expect("valid fraction");
        assert_eq!(result, expected);
    }
}
