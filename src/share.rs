use bls::{G2Affine, Scalar};
use ff::Field;
use rand_core::RngCore;

#[derive(Clone, Copy, Debug)]
pub struct Share {
    pub public: G2Affine,
    pub secret: Scalar,
}

impl Share {
    pub fn random<R: RngCore>(rng: &mut R) -> Self {
        let secret = Scalar::random(rng);
        Self {
            public: G2Affine::from(G2Affine::generator() * secret),
            secret,
        }
    }
}

pub struct ShareN<const N: usize> {
    pub public: [G2Affine; N],
    pub secret: [Scalar; N],
}

impl<const N: usize> ShareN<N> {
    pub fn random<R: RngCore>(rng: &mut R) -> Self {
        let mut share_n = Self {
            public: [G2Affine::identity(); N],
            secret: [Scalar::zero(); N],
        };
        for i in 0..N {
            let share = Share::random(rng);
            share_n.public[i] = share.public;
            share_n.secret[i] = share.secret;
        }
        share_n
    }

    pub fn update(&mut self, i: usize, with: Share) {
        self.public[i] = with.public;
        self.secret[i] = with.secret;
    }
}

pub struct CalculatedShare {
    pub share: Option<Share>,
    pub ph_items: Option<Vec<(Scalar, G2Affine)>>,
    pub pg: Option<G2Affine>,
}

#[cfg(test)]
mod test {
    use super::*;
    use rand_core::SeedableRng;
    use rand_xorshift::XorShiftRng;

    const SEED: [u8; 16] = [0; 16];

    #[test]
    fn gen_share() {
        let mut rng = XorShiftRng::from_seed(SEED);
        let mut share_n = ShareN::<5>::random(&mut rng);
        let old_share = Share::random(&mut rng);

        let i = 3usize;
        share_n.update(i, old_share.clone());
        assert_eq!(share_n.public[i], old_share.public);
        assert_eq!(share_n.secret[i], old_share.secret);
    }
}
