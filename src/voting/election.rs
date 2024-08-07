use super::ballot::Ballot;
use crate::blockchain::chain::Isonomia;

pub struct Election {
    blockchain: Isonomia,
}

impl Election {
    pub fn new() -> Self {
        Election {
            blockchain: Isonomia::new(),
        }
    }

    pub fn cast_vote(&mut self, ballot: Ballot) {
        let ballot_data = serde_json::to_string(&ballot).unwrap();
        self.blockchain.add_block(ballot_data);
    }

    pub fn get_results(&self) -> std::collections::HashMap<String, u64> {
        let mut results = std::collections::HashMap::new();
        for block in self.blockchain.get_chain().iter().skip(1) {
            if let Ok(ballot) = serde_json::from_str::<Ballot>(&block.data) {
                *results.entry(ballot.choice).or_insert(0) += 1;
            }
        }
        results
    }

    pub fn is_valid(&self) -> bool {
        self.blockchain.is_chain_valid()
    }
}
