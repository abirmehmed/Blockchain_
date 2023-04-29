
use std::fmt::Debug;
use std::time::{SystemTime, UNIX_EPOCH};
#[derive(Debug)]
struct Block {
    index: u32,
    timestamp: u128,
    data: String,
    previous_hash: String,
    hash: String,
}
impl Block {
    fn new(index: u32, data: String, previous_hash: String) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        let hash = calculate_hash(index, timestamp, &data, &previous_hash);
        Block {
            index,
            timestamp,
            data,
            previous_hash,
            hash,
        }
    }
}
fn calculate_hash(index: u32, timestamp: u128, data: &str, previous_hash: &str) -> String {
    format!("{}{}{}{}", index, timestamp, data, previous_hash)
}
#[derive(Debug)]
struct Blockchain {
    blocks: Vec<Block>,
}
impl Blockchain {
    fn new() -> Self {
        let genesis_block = Block::new(0, "Genesis block".to_string(), "0".to_string());
        Blockchain {
            blocks: vec![genesis_block],
        }
    }
    fn add_block(&mut self, data: String) {
        let previous_block = self.blocks.last().unwrap();
        let new_block = Block::new(
            previous_block.index + 1,
            data,
            previous_block.hash.to_string(),
        );
        self.blocks.push(new_block);
    }
}
fn main() {
    let mut blockchain = Blockchain::new();
    blockchain.add_block("First block".to_string());
    blockchain.add_block("Second block".to_string());
    println!("{:#?}", blockchain);
}
