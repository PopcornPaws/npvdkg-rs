use crate::polynomial::Polynomial;
use crate::pvsh::PvshProof;
use crate::{CalculatedShare, Participant, Share};

use bls::G2Affine;
use rand_core::RngCore;

pub struct Contribution {
    pub participant: Participant,
    pub proof: PvshProof,
    pub public_share: G2Affine,
}
pub struct Npvdkgrs<const N: usize, const T: usize>;

impl<const N: usize, const T: usize> Npvdkgrs<N, T> {
    const SIZE_CHECK: () = assert!(N >= T, "invalid threshold");

    pub fn contributions<R: RngCore>(
        rng: &mut R,
        participants: Vec<Participant>,
        me: Participant,
        old_share: Option<Share>,
    ) -> Vec<Contribution> {
        let _ = Self::SIZE_CHECK;

        let mut shares = [Share::random(rng); T];
        if let Some(share) = old_share {
            shares[0] = share;
        }

        let mut poly_coeffs = [Scalar::zero(); T];
        todo!();
    }
}

//    // "me" should be included in participants
//    let mut shares = Vec::<Share>::with_capacity(participants.len());
//
//    if let Some(sh) = old_share {
//        shares.push(sh);
//    }
//
//    while shares.len() < threshold {
//        shares.push(Share::random(rng));
//    }
//
//    for participant in participants.into_iter() {
//        // TODO lagrange interpolation
//    }
//
//    //let mut contribution = Contribution {
//    //    contributor: me,
//    //    contributions: Vec::<(Participant, PvshProof)>::with_capacity(participants.len()),
//    //};
//    todo!()
//}
//
//pub fn calculate_my_key() -> CalculatedShare {
//    todo!();
//}
