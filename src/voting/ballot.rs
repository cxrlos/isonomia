use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Ballot {
    pub voter_id: String,
    pub choice: String,
}

impl Ballot {
    pub fn new(voter_id: String, choice: String) -> Self {
        Ballot { voter_id, choice }
    }
}
