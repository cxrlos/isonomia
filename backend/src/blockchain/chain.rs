use super::block::Block;

pub struct Isonomia {
    chain: Vec<Block>,
}

impl Default for Isonomia {
    fn default() -> Self {
        Self::new()
    }
}

impl Isonomia {
    pub fn new() -> Self {
        let genesis_block = Block::new(0, String::from("Genesis Block"), String::from("0"));
        Isonomia {
            chain: vec![genesis_block],
        }
    }

    pub fn add_block(&mut self, data: String) {
        let previous_block = self.chain.last().expect("Chain can't be empty");
        let new_block = Block::new(
            self.chain.len() as u64,
            data,
            previous_block.hash.clone(),
        );
        self.chain.push(new_block);
    }

    pub fn is_chain_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current_block = &self.chain[i];
            let previous_block = &self.chain[i - 1];
            if current_block.hash != current_block.calculate_hash() {
                return false;
            }
            if current_block.previous_hash != previous_block.hash {
                return false;
            }
        }
        true
    }

    pub fn get_chain(&self) -> &Vec<Block> {
        &self.chain
    }
}
