use bls::{G1Affine, G2Affine, Scalar, pairing};
use ff::Field;
use rand_core::RngCore;

pub struct Participant {
    pub id: Scalar,
    pub pubkey: G2Affine,
}

pub struct Proof {
    pub c: Scalar,
    pub U: G2Affine,
    pub V: G1Affine,
}

impl Proof {
    pub fn pvsh_encode<R: RngCore>(rng: &mut R, participant: Participant, sh: Scalar) -> Self {
        let r = Scalar::random(rng);
        //let mut hasher = Sha3_256::new();
        //hasher
        //    .chain_update(participant.id.to_bytes())
        //    .chain_update(participant.pubkey.to_bytes());
        //let mut hash = [u8; 32];
        //hasher.finalize_into(&mut hash);
        //let Q = G1Affine::from_bytes(hash);

        //let e = pairing(Q, r * participant.pubkey);
        //let eh = 
        todo!();
    }
}

/*
#[allow(non_snake_case)]
pub fn pvsh_encode(id: &Fr, pubkey: &G2, share: &Fr, g2: &G2) -> String {
    // select random point in Fr
    let mut r = Fr::from_csprng();

    // compute "Q" value
    let mut q_preimage = id.serialize_raw().unwrap();
    q_preimage.append(&mut pubkey.serialize_raw().unwrap());
    let mut Q = G1::hash_and_map(&q_preimage).unwrap();

    // compute "eh" from the pairing
    let pubkey_r = pubkey * r;
    let eh_preimage = GT::from_pairing(&Q, &pubkey_r);
    let mut hasher = Sha3_256::new();
    hasher.update(&eh_preimage.serialize_raw().unwrap());
    let hash = &hasher.finalize()[..];
    let hash_str = std::str::from_utf8(hash).unwrap();
    let eh = Fr::from_str(hash_str, Base::Hex);

    // compute ciphertext "c"
    let mut c = share + eh;

    // compute "U" value
    let mut U = g2 * r;

    // variables as serialized strings
    let c_string = serde_json::to_string(&c).unwrap();
    let U_string = serde_json::to_string(&U).unwrap();
    let Q_string = serde_json::to_string(&Q).unwrap();
    // compute "H" value
    let h_preimage = format!("{}.{}.{}", &Q_string, &c_string, &U_string);
    let mut hasher = Sha3_256::new();
    hasher.update(h_preimage.as_bytes());
    let hash = &hasher.finalize()[..];
    let H = G1::hash_and_map(hash).unwrap();

    // compute verification vector "V"
    let quotient = eh / r;
    let mut V = &H * quotient;

    // compute resulting proof
    let V_string = serde_json::to_string(&V).unwrap();
    let proof = format!("{}.{}.{}", &c_string, &U_string, &V_string);

    r.clear();
    Q.clear();
    c.clear();
    U.clear();
    V.clear();

    proof
}

#[allow(non_snake_case)]
pub fn pvsh_verify(id: &Fr, pubkey: &G2, PH: &G2, proof: &str, g2: &G2) -> bool {
    let esh_array: Vec<&str> = proof.split('.').collect();
    assert!(esh_array.len() == 3, "invalid proof provided");
    // parse proof components
    let c = Fr::from_str(esh_array[0], Base::Hex);
    let U = G2::from_str(esh_array[1], Base::Hex);
    let V = G1::from_str(esh_array[2], Base::Hex);
    // compute "Q" value (TODO: should be an input parameter)
    let mut q_preimage = id.serialize_raw().unwrap();
    q_preimage.append(&mut pubkey.serialize_raw().unwrap());
    let Q = G1::hash_and_map(&q_preimage).unwrap();
    // compute "H" value
    let Q_string = serde_json::to_string(&Q).unwrap();
    let H = G1::hash_and_map(format!("{}.{}.{}", &Q_string, esh_array[0], esh_array[1]).as_bytes())
        .unwrap();
    // compute and verify pairings
    let g2c = g2 * c;
    let pairing_1 = GT::from_pairing(&H, &g2c);
    let pairing_2 = GT::from_pairing(&H, &PH);
    let pairing_3 = GT::from_pairing(&V, &U);

    let expected = &pairing_2 * pairing_3;

    pairing_1 == expected
}

#[allow(non_snake_case)]
pub fn pvsh_decode(id: &Fr, pubkey: &G2, sk: &Fr, proof: &str) -> Fr {
    let esh_array: Vec<&str> = proof.split('.').collect();
    assert!(esh_array.len() == 3, "invalid proof provided");
    let c = Fr::from_str(esh_array[0], Base::Hex);
    let U = G2::from_str(esh_array[1], Base::Hex);
    // compute "Q" value
    let mut q_preimage = id.serialize_raw().unwrap();
    q_preimage.append(&mut pubkey.serialize_raw().unwrap());
    let Q = G1::hash_and_map(&q_preimage).unwrap();
    // decode "eh"
    let pairing = GT::from_pairing(&(&Q * sk), &U);
    let mut hasher = Sha3_256::new();
    hasher.update(&pairing.serialize_raw().unwrap());
    let hash = &hasher.finalize()[..];
    let hash_str = std::str::from_utf8(hash).unwrap();
    let eh = Fr::from_str(hash_str, Base::Hex);

    c - eh
}
*/

//#[cfg(test)]
//mod test {
//    use super::*;
//    use mcl::init;
//
//    #[test]
//    fn pvsh_verify_and_decode() {
//        init::init_curve(init::Curve::Bls12_381);
//        let g2 = G2::hash_and_map(b"test generator").unwrap();
//        let id = Fr::from_csprng();
//        let sk = Fr::from_csprng();
//        let pk = &g2 * sk;
//        let sh = Fr::from_csprng();
//        let ph = &g2 * sh;
//        let proof = pvsh_encode(&id, &pk, &sh, &g2);
//        let result = pvsh_verify(&id, &pk, &ph, &proof, &g2);
//        let share = pvsh_decode(&id, &pk, &sk, &proof);
//
//        assert!(result);
//        assert_eq!(share, sh);
//    }
//}
