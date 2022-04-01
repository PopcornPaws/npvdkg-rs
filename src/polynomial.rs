// NOTE I couldn't find a usable lagrange interpolation library,
// so I'm just quickly concocting something to use temporarily

use bls::Scalar;
use ff::Field;
use rand_core::RngCore;

pub fn evaluate(coeffs: &[Scalar], at: Scalar) -> Scalar {
    coeffs
        .iter()
        .enumerate()
        .fold(Scalar::zero(), |acc, (i, x)| {
            let quotient = &[i as u64, 0, 0, 0]; // little endian repr.
            acc + x * at.pow(quotient)
        })
}

pub struct Point {
    x: Scalar,
    y: Scalar,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn poly_eval() {
        let at = Scalar::from_raw([2, 0, 0, 0]);
        let eval = evaluate(&[Scalar::one(); 5], at);
        // 1 + 1 * 2^1 + 1 * 2^2 + 1 * 2^3 + 1 * 2^4
        assert_eq!(eval, Scalar::from_raw([31, 0, 0, 0]));

        let coeffs = [
            Scalar::from_raw([5, 0, 0, 0]),
            Scalar::from_raw([10, 0, 0, 0]),
            Scalar::zero(),
            Scalar::from_raw([2, 0, 0, 0]),
        ];

        let at = Scalar::from_raw([5, 0, 0, 0]);
        let eval = evaluate(&coeffs, at);
        // 5 + 10 * 5^1 + 0 * 5^2 + 2 * 5^3
        assert_eq!(eval, Scalar::from_raw([305, 0, 0, 0]));
    }
}
