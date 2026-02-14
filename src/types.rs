use clickhouse::Row;
use near_primitives::hash::CryptoHash;
use near_primitives::types::BlockHeight;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

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

    pub tx_block_height: u64,
    pub tx_block_timestamp: u64,
    pub tx_index: u32,

    pub is_signer: bool,
    pub is_delegated_signer: bool,
    pub is_real_signer: bool,
    pub is_any_signer: bool,
    pub is_predecessor: bool,
    pub is_explicit_refund_to: bool,
    pub is_receiver: bool,
    pub is_real_receiver: bool,
    pub is_function_call: bool,
    pub is_action_arg: bool,
    pub is_event_log: bool,
    pub is_success: bool,
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

#[serde_as]
#[derive(Row, Serialize, Deserialize, Clone, Debug)]
pub struct BlockRow {
    pub block_height: u64,
    pub prev_block_height: Option<u64>,
    pub block_hash: String,
    pub prev_block_hash: String,
    #[serde_as(serialize_as = "DisplayFromStr", deserialize_as = "_")]
    pub block_timestamp: u64,
    pub epoch_id: String,
    pub next_epoch_id: String,
    pub chunks_included: u64,
    pub author_id: String,
    pub protocol_version: u32,
    #[serde_as(serialize_as = "DisplayFromStr", deserialize_as = "_")]
    pub gas_price: u128,
    pub block_ordinal: Option<u64>,
    #[serde_as(serialize_as = "DisplayFromStr", deserialize_as = "_")]
    pub total_supply: u128,
    pub num_transactions: u32,
    pub num_receipts: u32,
    #[serde_as(serialize_as = "DisplayFromStr", deserialize_as = "_")]
    pub gas_burnt: u64,
    #[serde_as(serialize_as = "DisplayFromStr", deserialize_as = "_")]
    pub tokens_burnt: u128,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum BlockId {
    Hash(CryptoHash),
    Height(BlockHeight),
}
