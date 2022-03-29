use crate::{FP_BYTES, G2_BYTES};
use bls::{G2Affine, Scalar};
use ff::Field;
use rand_core::RngCore;

pub struct Participant {
    pub id: Scalar,
    pub pubkey: G2Affine,
}

impl Participant {
    pub fn to_bytes(&self) -> [u8; FP_BYTES + G2_BYTES] {
        let mut output = [0u8; FP_BYTES + G2_BYTES];
        output[0..FP_BYTES].copy_from_slice(&self.id.to_bytes());
        output[FP_BYTES..].copy_from_slice(&self.pubkey.to_compressed());
        output
    }
}

pub struct Member {
    pub participant: Participant,
    pub secret_key: Scalar,
    pub share: Option<Share>,
    pub pg: Option<G2Affine>,
}

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

pub struct CalculatedShare {
    pub share: Option<Share>,
    pub ph_items: Option<Vec<(Scalar, G2Affine)>>,
    pub pg: Option<G2Affine>,
}
