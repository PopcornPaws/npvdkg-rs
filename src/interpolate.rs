use bls::Scalar;

pub fn interpolate(x: &[Scalar], y: &[Scalar]) -> Result<Vec<Scalar>, String> {
    if x.len() != y.len() {
        return Err("input lengths not equal".to_string());
    }

    let n = x.len();

    let mut s = vec![Scalar::zero(); n];
    let mut coeffs = vec![Scalar::zero(); n];

    s.push(Scalar::one());
    s[n - 1] = -x[0];

    for (i, x_elem) in x.iter().enumerate().skip(1) {
        // TODO? modular add takes a reference to rhs (could take ownership
        // because of copy)
        #[allow(clippy::assign_op_pattern)]
        for j in n - 1 - i..n - 1 {
            s[j] = s[j] - *x_elem * s[j + 1];
        }
        s[n - 1] -= *x_elem;
    }

    for i in 0..n {
        let mut phi = Scalar::zero();
        for j in (1..=n).rev() {
            phi = Scalar::from(j as u64) * s[j] + x[i] * phi;
        }
        // TODO unwrap?
        let ff = phi.invert().unwrap();
        let mut b = Scalar::one();
        for j in (0..n).rev() {
            coeffs[j] += b * ff * y[i];
            b = s[j] + x[i] * b;
        }
    }

    Ok(coeffs)
}

#[cfg(test)]
mod test {
    use super::*;
    use std::ops::Neg;

    //fn eval_poly<C: Curve>(coeffs: &[Scalar<C>], x: Scalar<C>) -> Scalar<C> {
    //    let mut ret = Scalar::ZERO;
    //    for coeff in coeffs.iter().rev() {
    //        ret = *coeff + x * ret;
    //    }
    //    ret
    //}

    #[test]
    fn interpolate_polynomial() {
        // not equal length inputs
        let x = vec![Scalar::from(3_u64); 3];
        let y = vec![Scalar::from(5_u64); 4];
        assert!(interpolate(&x, &y).is_err());

        // constant polynomial (y = 53)
        let x = vec![Scalar::from(3_u64); 1];
        let y = vec![Scalar::from(53_u64); 1];
        let coeffs = interpolate(&x, &y).unwrap();
        assert_eq!(coeffs[0], Scalar::from(53_u64));

        // simple first order polynomial (y = x)
        let x = vec![
            Scalar::from(1_u64),
            Scalar::from(2_u64),
            Scalar::from(3_u64),
        ];

        let y = x.clone();
        let coeffs = interpolate(&x, &y).unwrap();
        assert_eq!(coeffs[0], Scalar::zero()); // c_0
        assert_eq!(coeffs[1], Scalar::one()); // c_1
        assert_eq!(coeffs[2], Scalar::zero()); // c_2

        // first order polynomial (y = 32 * x - 13)
        let x = vec![Scalar::from(2_u64), Scalar::from(3_u64)];
        let y = vec![Scalar::from(51_u64), Scalar::from(83_u64)];
        let coeffs = interpolate(&x, &y).unwrap();
        assert_eq!(coeffs[0], Scalar::from(13_u64).neg());
        assert_eq!(coeffs[1], Scalar::from(32_u64));

        // fourth order polynomial
        // y = x^4 + 0 * x^3 + 3 * x^2 + 2 * x + 14
        let x = vec![
            Scalar::from(1_u64),
            Scalar::from(2_u64),
            Scalar::from(3_u64),
            Scalar::from(4_u64),
            Scalar::from(5_u64),
            Scalar::from(6_u64),
        ];
        let y = vec![
            Scalar::from(20_u64),
            Scalar::from(46_u64),
            Scalar::from(128_u64),
            Scalar::from(326_u64),
            Scalar::from(724_u64),
            Scalar::from(1430_u64),
        ];
        let coeffs = interpolate(&x, &y).unwrap();
        assert_eq!(coeffs[0], Scalar::from(14_u64)); // c0 (x^0)
        assert_eq!(coeffs[1], Scalar::from(2_u64)); // c1 (x^1)
        assert_eq!(coeffs[2], Scalar::from(3_u64)); // c2 (x^2)
        assert_eq!(coeffs[3], Scalar::from(0_u64)); // c3 (x^3)
        assert_eq!(coeffs[4], Scalar::from(1_u64)); // c4 (x^4)
        assert_eq!(coeffs[5], Scalar::from(0_u64)); // c5 (x^5)
    }
}
