// NOTE I couldn't find a usable lagrange interpolation library,
// so I'm just quickly concocting something to use temporarily

use bls::Scalar;
use ff::Field;
use rand_core::RngCore;

// Is this really necessary?
// DEG - polynomial degree
pub struct Polynomial<const DEG: usize>([Scalar; DEG + 1])
where
    [(); DEG + 1]: Sized;

impl<const DEG: usize> Polynomial<DEG>
where
    [(); DEG + 1]: Sized,
{
    pub fn new<R: RngCore>(rng: &mut R, secret: Scalar) -> Self {
        // fill coeffs with secret
        let mut coeffs = [secret; DEG + 1];
        // override elements with a random coeff
        // 0^th element remains the secret
        for i in 1..DEG {
            coeffs[i] = Scalar::random(&mut *rng);
        }
        Self(coeffs)
    }

    pub fn evaluate(&self, at: Scalar) -> Scalar {
        self.0
            .iter()
            .enumerate()
            .fold(Scalar::zero(), |acc, (i, x)| {
                let quotient = &[i as u64, 0, 0, 0]; // little endian repr.
                acc + x.pow(quotient)
            })
    }
}

pub struct Point {
    x: Scalar,
    y: Scalar,
}

pub fn recover(points: Vec<Point>) -> Scalar {
    todo!()
}
