use mcl::bn::{Fr, G1, G2, GT};
use mcl::common::Base;
use mcl::traits::RawSerializable;
use sha3::{Digest, Sha3_256};

#[allow(non_snake_case)]
pub fn pvsh_encode(id: Fr, pubkey: G2, share: Fr, g2: G2) -> String {
    // select random point in Fr
    let mut r = Fr::from_csprng();

    // compute "Q" value
    let mut q_preimage = id.serialize_raw().unwrap();
    q_preimage.append(&mut pubkey.serialize_raw().unwrap());
    let mut Q = G1::hash_and_map(&q_preimage).unwrap();

    // compute "eh" from the pairing
    let pubkey_r = &pubkey * r;
    let eh_preimage = GT::from_pairing(&Q, &pubkey_r);
    let mut hasher = Sha3_256::new();
    hasher.update(&eh_preimage.serialize_raw().unwrap());
    let hash = &hasher.finalize()[..];
    let hash_str = std::str::from_utf8(hash).unwrap();
    let eh = Fr::from_str(hash_str, Base::Hex);

    // compute ciphertext "c"
    let mut c = share + eh;

    // compute "U" value
    let mut U = &g2 * r;

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
pub fn pvsh_verifiy(id: Fr, pubkey: G2, pubhash: G2, proof: &str, g2: G2) -> bool {
    let esh_array = proof.split('.').collect();
    assert!(esh_array.len() == 3, "invalid proof provided");
    // compute "Q" value
    let mut q_preimage = id.serialize_raw().unwrap();
    q_preimage.append(&mut pubkey.serialize_raw().unwrap());
    let mut Q = G1::hash_and_map(&q_preimage).unwrap();

    todo!();
}
