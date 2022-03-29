use crate::{Participant, CalculatedShare, Share};
use crate::pvsh::PvshProof;

use bls::G2Affine;
use rand_core::RngCore;

pub struct Contribution {
    pub contributor: Participant,
    pub contributions: Vec<(Participant, PvshProof)>,
    pub public_shares: Vec<G2Affine>,
}

pub fn calculate_contribution<R: RngCore>(
    rng: &mut R,
    threshold: usize,
    me: Participant,
    participants: Vec<Participant>,
    old_share: Option<Share>,
) -> Contribution {
    assert!(
        threshold <= participants.len(),
        "threshold cannot be greater than the nubmer of participants"
    );

    // "me" should be included in participants
    let mut shares = Vec::<Share>::with_capacity(participants.len());

    if let Some(sh) = old_share {
        shares.push(sh);
    }

    while shares.len() < threshold {
        shares.push(Share::random(rng));
    }

    for participant in participants.into_iter() {
        // TODO lagrange interpolation
    }

    //let mut contribution = Contribution {
    //    contributor: me,
    //    contributions: Vec::<(Participant, PvshProof)>::with_capacity(participants.len()),
    //};
    todo!()
}

pub fn calculate_my_key() -> CalculatedShare {
    todo!();
}
