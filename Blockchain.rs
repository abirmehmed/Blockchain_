
// Import the necessary modules from the standard library
use std::fmt::Debug;
use std::time::{SystemTime, UNIX_EPOCH};
// Define a Block struct with fields for the index, timestamp, data, previous hash, and hash
#[derive(Debug)]
struct Block {
    index: u32,
    timestamp: u128,
    data: String,
    previous_hash: String,
    hash: String,
}
// Implement methods for the Block struct
impl Block {
    // Define a new method to create a new Block
    fn new(index: u32, data: String, previous_hash: String) -> Self {
        // Get the current timestamp in milliseconds since the UNIX epoch
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        // Calculate the hash of the block using the calculate_hash function
        let hash = calculate_hash(index, timestamp, &data, &previous_hash);
        // Return a new Block with the given index, timestamp, data, previous hash, and calculated hash
        Block {
            index,
            timestamp,
            data,
            previous_hash,
            hash,
        }
    }
}
// Define a function to calculate the hash of a block
fn calculate_hash(index: u32, timestamp: u128, data: &str, previous_hash: &str) -> String {
    // Concatenate the index, timestamp, data, and previous hash and return the resulting string as the hash
    format!("{}{}{}{}", index, timestamp, data, previous_hash)
}
// Define a Blockchain struct with a field for the blocks in the chain
#[derive(Debug)]
struct Blockchain {
    blocks: Vec<Block>,
}
// Implement methods for the Blockchain struct
impl Blockchain {
    // Define a new method to create a new Blockchain
    fn new() -> Self {
        // Create the genesis block with an index of 0 and a previous hash of "0"
        let genesis_block = Block::new(0, "Genesis block".to_string(), "0".to_string());
        // Return a new Blockchain with the genesis block as its only block
        Blockchain {
            blocks: vec![genesis_block],
        }
    }
    // Define an add_block method to add a new block to the chain
    fn add_block(&mut self, data: String) {
        // Get the last block in the chain
        let previous_block = self.blocks.last().unwrap();
        // Create a new block with an index one greater than the previous block's index and using the given data and previous block's hash
        let new_block = Block::new(
            previous_block.index + 1,
            data,
            previous_block.hash.to_string(),
        );
        // Add the new block to the chain
        self.blocks.push(new_block);
    }
}
// Define a main function to run the code
fn main() {
    // Create a new blockchain
    let mut blockchain = Blockchain::new();
    // Add two blocks to the blockchain with custom data
    blockchain.add_block("First block".to_string());
    blockchain.add_block("Second block".to_string());
    // Print out the blockchain to see its contents
    println!("{:#?}", blockchain);
}
