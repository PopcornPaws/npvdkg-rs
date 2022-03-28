pub type Id = ;
pub type PublicKey = ;
pub type SecretKey = ;

pub struct Participant {
    pub id: Id,
    pub pubkey: PublicKey,
}

pub struct ParticipantShare {
    pub id: Id,
    pub ph: PublicKey,
}

pub struct Member {
    pub participant: Participant,
    /// Secret key
    pub sk: SecretKey,
    /// Secret share
    pub sh: Option<SecretKey>,
    /// Public share
    pub ph: Option<PublicKey>,
    pub pg: Option<PublicKey>,
}

pub struct Contribution {
    pub contributor: Participant,
    pub contributions: Vec<(Participant, Proof)>,
    pub mkp_vec: Vec<String>,
}

// TODO let this have 3 fields (c, U, V)
pub struct Proof {
    pub esh: String,
}

pub struct CalculatedShare {
    sh: Option<String>,
    ph: Option<String>,
    ph_items: Option<Vec<ParticipantShare>>,
    pg: Option<PublicKey>,
}
