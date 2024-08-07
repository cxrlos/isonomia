use super::ballot::Ballot;
use crate::blockchain::chain::Isonomia;
use ed25519_dalek::Keypair;
use rand::rngs::OsRng;

pub struct Election {
    blockchain: Isonomia,
}

impl Election {
    pub fn new() -> Self {
        Election {
            blockchain: Isonomia::new(),
        }
    }

    pub fn cast_vote(&mut self, voter_id: String, choice: String) -> Result<(), String> {
        let mut csprng = OsRng;
        let keypair = Keypair::generate(&mut csprng);
        
        let ballot = Ballot::new(voter_id, choice, &keypair);
        
        if ballot.verify() {
            let ballot_data = serde_json::to_string(&ballot).unwrap();
            self.blockchain.add_block(ballot_data);
            Ok(())
        } else {
            Err("Invalid ballot signature".to_string())
        }
    }

    pub fn get_results(&self) -> std::collections::HashMap<String, u64> {
        let mut results = std::collections::HashMap::new();
        for block in self.blockchain.get_chain().iter().skip(1) {
            if let Ok(ballot) = serde_json::from_str::<Ballot>(&block.data) {
                if ballot.verify() {
                    *results.entry(ballot.choice).or_insert(0) += 1;
                }
            }
        }
        results
    }

    pub fn is_valid(&self) -> bool {
        self.blockchain.is_chain_valid()
    }
}
