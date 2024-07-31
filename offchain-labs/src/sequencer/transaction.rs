use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Transaction {
    pub sender: String,
    pub recipient: String,
    pub amount: Vec<u8>,
    pub nonce: u64,
}

impl Transaction {
    pub fn new(sender: String, recipient: String, amount: Vec<u8>, nonce: u64) -> Self {
        Self { sender, recipient, amount, nonce }
    }
}