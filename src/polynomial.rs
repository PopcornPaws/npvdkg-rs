// NOTE I couldn't find a usable lagrange interpolation library,
// so I'm just quickly concocting something to use temporarily

use bls::Scalar;
use ff::Field;
use rand_core::RngCore;

// N = DEG + 1, i.e. the number of coefficients is the
// polynomial degree + 1
// NOTE const generics would make the implementation
// NOTE more elegant as some constraints would be enforced at
// NOTE compile time, however, it might be heavy on the stack
// NOTE and make implementation more complicated
pub struct Polynomial<const N: usize>([Scalar; N]);

impl<const N: usize> Polynomial<N> {
    const SIZE_CHECK: () = assert!(N <= 21, "too large degree");

    pub fn new(coeffs: [Scalar; N]) -> Self {
        let _ = Self::SIZE_CHECK;
        Self(coeffs)
    }

    pub fn evaluate(&self, at: Scalar) -> Scalar {
        self.0
            .iter()
            .enumerate()
            .fold(Scalar::zero(), |acc, (i, x)| {
                let quotient = &[i as u64, 0, 0, 0]; // little endian repr.
                acc + x * at.pow(quotient)
            })
    }

    pub fn recover(points: [Point; N]) -> Scalar {
        let _ = Self::SIZE_CHECK;
        todo!();
    }

    pub fn as_mut_slice(&mut self) -> &mut [Scalar; N] {
        &mut self.0
    }
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
        let poly = Polynomial::<5>::new([Scalar::one(); 5]);
        let at = Scalar::from_raw([2, 0, 0, 0]);
        let eval = poly.evaluate(at);
        // 1 + 1 * 2^1 + 1 * 2^2 + 1 * 2^3 + 1 * 2^4
        assert_eq!(eval, Scalar::from_raw([31, 0, 0, 0]));

        let poly = Polynomial::<4>::new([
            Scalar::from_raw([5, 0, 0, 0]),
            Scalar::from_raw([10, 0, 0, 0]),
            Scalar::zero(),
            Scalar::from_raw([2, 0, 0, 0]),
        ]);

        let at = Scalar::from_raw([5, 0, 0, 0]);
        let eval = poly.evaluate(at);
        // 5 + 10 * 5^1 + 0 * 5^2 + 2 * 5^3
        assert_eq!(eval, Scalar::from_raw([305, 0, 0, 0]));
    }

    #[test]
    fn secret_recover() {}
}
