use clickhouse::Row;
use near_primitives::hash::CryptoHash;
use near_primitives::types::BlockHeight;
use serde::{Deserialize, Serialize};

#[derive(Row, Serialize, Deserialize, Clone, Debug)]
pub struct TransactionRow {
    pub transaction_hash: String,
    pub signer_id: String,
    pub tx_block_height: u64,
    pub tx_block_hash: String,
    pub tx_block_timestamp: u64,
    pub transaction: String,
    pub last_block_height: u64,
}

#[derive(Row, Serialize, Deserialize, Clone, Debug)]
pub struct AccountTxRow {
    pub account_id: String,
    pub transaction_hash: String,
    pub signer_id: String,
    pub tx_block_height: u64,
    pub tx_block_timestamp: u64,
}

#[derive(Row, Serialize, Deserialize, Clone, Debug)]
pub struct BlockTxRow {
    pub block_height: u64,
    pub block_hash: String,
    pub block_timestamp: u64,
    pub transaction_hash: String,
    pub signer_id: String,
    pub tx_block_height: u64,
}

#[derive(Row, Serialize, Deserialize, Clone, Debug)]
pub struct ReceiptTxRow {
    pub receipt_id: String,
    pub transaction_hash: String,
    pub signer_id: String,
    pub tx_block_height: u64,
    pub tx_block_timestamp: u64,
}

#[derive(Row, Deserialize, Serialize, Debug, Clone)]
pub struct BlocksWithTxCount {
    pub block_height: u64,
    pub txs_count: u64,
    pub block_timestamp: u64,
    pub block_hash: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum BlockId {
    Hash(CryptoHash),
    Height(BlockHeight),
}
