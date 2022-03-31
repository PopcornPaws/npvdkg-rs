// NOTE I couldn't find a usable lagrange interpolation library,
// so I'm just quickly concocting something to use temporarily

use bls::Scalar;
use ff::Field;
use rand_core::RngCore;

pub fn share<const DEG: usize, R: RngCore>(rng: &mut R, secret: Scalar) -> Scalar {
    // fill coeffs with secret
    let mut coeffs = [secret; DEG + 1];
    // override elements with a random coeff
    // 0^th element remains the secret
    for i in 1..DEG {
        coeffs[i] = Scalar::random(&mut *rng);
    }
    todo!();
}

//    pub fn evaluate(&self, at: Scalar) -> Scalar {
//        self.0
//            .iter()
//            .enumerate()
//            .fold(Scalar::zero(), |acc, (i, x)| {
//                let quotient = &[i as u64, 0, 0, 0]; // little endian repr.
//                acc + x.pow(quotient)
//            })
//    }

pub struct Point {
    x: Scalar,
    y: Scalar,
}

pub fn recover(points: Vec<Point>) -> Scalar {
    todo!()
}
