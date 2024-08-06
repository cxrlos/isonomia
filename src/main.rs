use chrono::Utc;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

// Block struct
#[derive(Debug, Serialize, Deserialize, Clone)]
struct Block {
    index: u64,
    timestamp: i64,
    data: String,
    previous_hash: String,
    hash: String,
}

impl Block {
    fn new(index: u64, data: String, previous_hash: String) -> Self{
        let timestamp = Utc::now().timestamp();
        let mut block = Block {
            index,
            timestamp,
            data,
            previous_hash,
            hash: String::new(),
        };
        block.hash = block.calculate_hash();
        block
    }
    
    // Encapsulate the hash function
    fn calculate_hash(&self) -> String {
        let mut hasher = Sha256::new();
        // Concatenate the block fields using update method for performance
        hasher.update(self.index.to_string());
        hasher.update(self.timestamp.to_string());
        hasher.update(&self.data);
        hasher.update(&self.previous_hash);
        format!("{:x}", hasher.finalize())
    }
}

// Main struct to manage the chain
struct Isonomia {
    chain: Vec<Block>,
}

impl Isonomia {
    fn new() -> Self {
        let genesis_block = Block::new(0, String::from("Genesis Block"), String::from("0"));
        Isonomia {
            chain: vec![genesis_block],
        }
    }

    // Add a new block to the chain
    fn add_block(&mut self, data: String) {
        let previous_block = self.chain.last().expect("Chain can't be empty");
        let new_block = Block::new(
            self.chain.len() as u64,
            data,
            previous_block.hash.clone(),
        );
        self.chain.push(new_block);
    }

    // Check if the chain is valid
    fn is_chain_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current_block = &self.chain[i];
            let previous_block = &self.chain[i - 1];
            // Check if the hash of the block is correct
            if current_block.hash != current_block.calculate_hash() {
                return false;
            }
            // Check if the previous_hash field is correct
            if current_block.previous_hash != previous_block.hash {
                return false;
            }
        }
        true
    }
}

fn main() {
    let mut isonomia = Isonomia::new();
    isonomia.add_block(String::from("First Block"));
    isonomia.add_block(String::from("Second Block"));
    isonomia.add_block(String::from("Third Block"));

    println!("Is the chain valid? {}", isonomia.is_chain_valid());

    for block in isonomia.chain {
        println!("{:?}", block);
    }
}
