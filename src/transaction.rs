use hashbrown::HashMap;
use serde::Deserialize;

use crate::utils::arbitrary_tx_amount;

/// Transaction type enum
#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[repr(u8)]
pub enum TransactionKind {
    Deposit,
    Withdrawal,
    Dispute,
    Resolve,
    Chargeback,
}

/// Minimal internal transaction data
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct TransactionData {
    /// Transaction type
    #[serde(rename = "type")]
    pub kind: TransactionKind,

    /// Transaction amount
    // TODO: Remove this field for _advanced_ txs as it's currently redundant for them
    #[serde(default, deserialize_with = "arbitrary_tx_amount")]
    pub amount: f32,
}

/// Deserialized transaction
#[derive(Deserialize, Debug)]
pub struct Transaction {
    /// Client ID
    #[serde(rename = "client")]
    pub client_id: u16,

    /// Transaction ID
    #[serde(rename = "tx")]
    pub id: u32,

    /// Transaction data
    #[serde(flatten)]
    transaction_data: TransactionData,
}

impl Transaction {
    /// Returns a copy of internal transaction data
    ///
    /// This is reasonably cheap as one transaction data is roughly 8 bytes
    /// And the base Transaction is dropped after deserialization
    // TODO: Return tx data by reference instead of cloning
    pub fn get_data(&self) -> TransactionData {
        self.transaction_data.clone()
    }
}

/// A map of transactions' data with transaction ID as a key
pub type Transactions = HashMap<u32, TransactionData>;
