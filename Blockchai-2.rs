use std::fmt::Debug;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

#[derive(Debug)]
struct Block<T> {
    index: u64,
    timestamp: u64,
    data: T,
    previous_hash: u64,
    hash: u64,
}

impl<T> Block<T>
where
    T: Hash + Debug,
{
    fn new(index: u64, timestamp: u64, data: T, previous_hash: u64) -> Self {
        let mut block = Block {
            index,
            timestamp,
            data,
            previous_hash,
            hash: 0,
        };
        block.hash = block.calculate_hash();
        block
    }

    fn calculate_hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.index.hash(&mut hasher);
        self.timestamp.hash(&mut hasher);
        self.data.hash(&mut hasher);
        self.previous_hash.hash(&mut hasher);
        hasher.finish()
    }
}

#[derive(Debug)]
struct Blockchain<T> {
    blocks: Vec<Block<T>>,
}

impl<T> Blockchain<T>
where
    T: Hash + Debug + Copy,
{
    fn new(data: T) -> Self {
        let genesis_block = Block::new(0, 0, data, 0);
        Blockchain {
            blocks: vec![genesis_block],
        }
    }

    fn add_block(&mut self, timestamp: u64, data: T) {
        let previous_block = &self.blocks[self.blocks.len() - 1];
        let block = Block::new(
            previous_block.index + 1,
            timestamp,
            data,
            previous_block.hash,
        );
        self.blocks.push(block);
    }
}

fn main() {
    let mut blockchain = Blockchain::new("Genesis block");
    blockchain.add_block(1, "Block 1");
    blockchain.add_block(2, "Block 2");
    println!("{:#?}", blockchain);
}
