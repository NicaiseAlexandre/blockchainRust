use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::ptr::hash;
use std::time::SystemTime;
use rand::Rng;
use sha2::{Sha256, Sha512, Digest};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Transaction {
    sender: String,
    recipient: String,
    amount: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Block {
    id: u64,
    timestamp: u64,
    transactions: Vec<Transaction>,
    proof: u64,
    previous_hash: String,
    nonce: u64,
}

impl Block {
    fn new(id: u64, transactions: Vec<Transaction>, proof: u64, previous_hash: String, nonce: u64) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("Time error")
            .as_secs();
        Block {
            id,
            timestamp,
            transactions,
            proof,
            previous_hash,
            nonce,
        }
    }

    fn hash(&self) -> String {
        let block_string = serde_json::to_string(&self).unwrap();
        //TODO  :(
        let mut hasher = Sha256::new();
        hasher.input_str(&block_string);
        hasher.result_str()
    }
}

struct Blockchain {
    chain: Vec<Block>,
    current_transactions: Vec<Transaction>,
    nodes: HashMap<String, u64>,
}

impl Blockchain {
    fn new() -> Self {
        let genesis_block = Block::new(0, vec![], 0, String::from("0"), 0);
        let chain = vec![genesis_block];
        Blockchain {
            chain,
            current_transactions: vec![],
            nodes: HashMap::new(),
        }
    }

    fn add_transaction(&mut self, sender: String, recipient: String, amount: u64) -> u64 {
        self.current_transactions.push(Transaction {
            sender,
            recipient,
            amount,
        });
        self.chain.last().expect("Chain is empty").id + 1
    }

    fn add_block(&mut self, proof: u64, nonce: u64) -> Block {
        let previous_hash = self.chain.last().expect("Chain is empty").hash();
        let block = Block::new(
            self.chain.len() as u64,
            self.current_transactions.clone(),
            proof,
            previous_hash,
            nonce,
        );
        self.current_transactions.clear();
        self.chain.push(block.clone());
        block
    }
}

fn main() {
    let mut blockchain = Blockchain::new();

    blockchain.add_transaction("Alice".to_string(), "Bob".to_string(), 50);
    blockchain.add_transaction("Bob".to_string(), "Charlie".to_string(), 25);

    let proof = 12345;
    let nonce = 67890;

    let block1 = blockchain.add_block(proof, nonce);

}

