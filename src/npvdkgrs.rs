use crate::participant::*;
use crate::polynomial::evaluate;
use crate::pvsh::PvshProof;
use crate::share::*;

use bls::{G2Affine, Scalar};
use rand_core::RngCore;

use std::mem::{self, MaybeUninit};

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
        participants: [Participant; N],
        me: Participant,
        old_share: Option<Share>,
    ) -> [Contribution; N] {
        let _ = Self::SIZE_CHECK;

        let mut shares = ShareN::<T>::random(rng);
        if let Some(share) = old_share {
            shares.update(0, share);
        }

        // NOTE MaybeUninit might be used to initialize a fixed
        // [Contribution; N] array element by element
        let mut contributions: [Contribution; N] = unsafe { MaybeUninit::uninit().assume_init() };

        for (participant, contribution) in &participants.zip(contributions){
            // TODO evaluate polynomial
            let secret_share = evaluate(&shares.secret, participant.id);
            let proof = PvshProof::encode(rng, &participant, &secret_share);
            contribution.write(Contribution {
                participant,
                proof,
                public_share: todo!(), // is this a vector?
            });
        }
        unsafe { mem::transmute::<_, [Contribution; N]>(contribution) }
        contributions
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
