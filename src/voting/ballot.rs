use serde::{Deserialize, Serialize};
use ed25519_dalek::{Keypair, PublicKey, Signature, Signer, Verifier};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Ballot {
    pub voter_id: String,
    pub choice: String,
    pub public_key: Vec<u8>,
    pub signature: Vec<u8>,
}

impl Ballot {
    pub fn new(voter_id: String, choice: String, keypair: &Keypair) -> Self {
        let message = format!("{}:{}", voter_id, choice);
        let signature = keypair.sign(message.as_bytes());
    
        Ballot {
            voter_id,
            choice,
            public_key: keypair.public.to_bytes().to_vec(),
            signature: signature.to_bytes().to_vec(),
        }
    }
    pub fn verify(&self) -> bool {
        let public_key = PublicKey::from_bytes(&self.public_key).expect("Error deserializing public key");
        let signature = Signature::from_bytes(&self.signature).expect("Error deserializing signature");
        let message = format!("{}:{}", self.voter_id, self.choice);
        
        public_key.verify(message.as_bytes(), &signature).is_ok()
    }
}
