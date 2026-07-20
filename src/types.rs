use clickhouse::Row;
use near_primitives::hash::CryptoHash;
use near_primitives::types::{AccountId, BlockHeight};
use serde::{Deserialize, Serialize};
use serde_json::value::RawValue;
use serde_with::{serde_as, DisplayFromStr};

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "openapi", schemars(deny_unknown_fields))]
pub struct TxInput {
    /// Up to 20 base58-encoded transaction hashes to fetch in one request.
    #[cfg_attr(feature = "openapi", schemars(schema_with = "tx_hashes_schema"))]
    pub tx_hashes: Vec<CryptoHash>,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "openapi", schemars(deny_unknown_fields))]
pub struct TransactionsResponse {
    #[cfg_attr(
        feature = "openapi",
        schemars(schema_with = "raw_transaction_array_schema")
    )]
    pub transactions: Vec<Box<RawValue>>,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "openapi", schemars(deny_unknown_fields))]
pub struct AccountInput {
    /// NEAR account to query transactions for (may be a signer, predecessor, receiver, or related party).
    #[cfg_attr(feature = "openapi", schemars(with = "String"))]
    pub account_id: AccountId,
    /// Restrict to transactions where this account signed the top-level transaction.
    pub is_signer: Option<bool>,
    /// Restrict to transactions where this account signed a delegated action.
    pub is_delegated_signer: Option<bool>,
    /// Restrict to transactions where this account was the real signer — direct or delegated, excluding relayer signers.
    pub is_real_signer: Option<bool>,
    /// Restrict to transactions where this account signed either the top-level transaction or a delegated action.
    pub is_any_signer: Option<bool>,
    /// Restrict to transactions where this account was the predecessor of a receipt.
    pub is_predecessor: Option<bool>,
    /// Restrict to transactions where this account was the explicit `refund_to` target of an action receipt.
    pub is_explicit_refund_to: Option<bool>,
    /// Restrict to transactions where this account received a receipt.
    pub is_receiver: Option<bool>,
    /// Restrict to transactions where this account was the real receiver — excluding relayer receivers and gas refunds.
    pub is_real_receiver: Option<bool>,
    /// Restrict to transactions where this account was the target of a function-call action.
    pub is_function_call: Option<bool>,
    /// Restrict to transactions where this account appeared in action arguments.
    pub is_action_arg: Option<bool>,
    /// Restrict to transactions where this account appeared in a JSON event log.
    pub is_event_log: Option<bool>,
    /// Restrict to transactions whose execution succeeded (true) or failed/pending (false).
    pub is_success: Option<bool>,
    /// Opaque pagination token returned on a prior page; omit for the first page.
    #[serde_as(as = "Option<DisplayFromStr>")]
    #[cfg_attr(feature = "openapi", schemars(with = "Option<String>"))]
    pub resume_token: Option<u128>,
    /// Inclusive lower bound on the transaction's block height.
    #[cfg_attr(feature = "openapi", schemars(range(min = 0)))]
    pub from_tx_block_height: Option<BlockHeight>,
    /// Exclusive upper bound on the transaction's block height.
    #[cfg_attr(feature = "openapi", schemars(range(min = 0)))]
    pub to_tx_block_height: Option<BlockHeight>,
    /// Maximum rows to return in one page (1–200).
    #[cfg_attr(feature = "openapi", schemars(range(min = 1, max = 200)))]
    pub limit: Option<usize>,
    /// Sort newest-first when true; oldest-first when false or omitted.
    pub desc: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "openapi", schemars(deny_unknown_fields))]
pub struct AccountResponse {
    pub account_txs: Vec<AccountTxRow>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "openapi", schemars(schema_with = "string_schema"))]
    pub resume_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "openapi", schemars(schema_with = "u64_schema"))]
    pub txs_count: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "openapi", schemars(deny_unknown_fields))]
pub struct BlockInput {
    /// Block height (integer) or base58-encoded block hash (string) to fetch.
    #[cfg_attr(feature = "openapi", schemars(schema_with = "block_id_schema"))]
    pub block_id: BlockId,
    /// Include the block's transactions in the response when true.
    pub with_transactions: Option<bool>,
    /// Include the block's receipts in the response when true.
    pub with_receipts: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "openapi", schemars(deny_unknown_fields))]
pub struct BlockResponse {
    #[cfg_attr(feature = "openapi", schemars(required))]
    #[cfg_attr(
        feature = "openapi",
        schemars(schema_with = "nullable_block_row_schema")
    )]
    pub block: Option<BlockRow>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "openapi", schemars(schema_with = "block_txs_schema"))]
    pub block_txs: Option<Vec<BlockTxRow>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "openapi", schemars(schema_with = "receipt_txs_schema"))]
    pub block_receipts: Option<Vec<ReceiptTxRow>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "openapi", schemars(deny_unknown_fields))]
pub struct BlocksInput {
    /// Inclusive lower bound on block height.
    #[cfg_attr(feature = "openapi", schemars(range(min = 0)))]
    pub from_block_height: Option<BlockHeight>,
    /// Exclusive upper bound on block height.
    #[cfg_attr(feature = "openapi", schemars(range(min = 0)))]
    pub to_block_height: Option<BlockHeight>,
    /// Maximum blocks to return in one page (1–100).
    #[cfg_attr(feature = "openapi", schemars(range(min = 1, max = 100)))]
    pub limit: Option<usize>,
    /// Sort newest-first when true; oldest-first when false or omitted.
    pub desc: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "openapi", schemars(deny_unknown_fields))]
pub struct BlocksResponse {
    pub blocks: Vec<BlockRow>,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "openapi", schemars(deny_unknown_fields))]
pub struct ReceiptInput {
    /// Base58-encoded receipt ID to look up.
    #[cfg_attr(feature = "openapi", schemars(with = "String"))]
    pub receipt_id: CryptoHash,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "openapi", schemars(deny_unknown_fields))]
pub struct ReceiptResponse {
    #[cfg_attr(feature = "openapi", schemars(required))]
    #[cfg_attr(
        feature = "openapi",
        schemars(schema_with = "nullable_receipt_tx_row_schema")
    )]
    pub receipt: Option<ReceiptTxRow>,
    #[cfg_attr(feature = "openapi", schemars(required))]
    #[cfg_attr(
        feature = "openapi",
        schemars(schema_with = "nullable_raw_transaction_schema")
    )]
    pub transaction: Option<Box<RawValue>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "openapi", schemars(deny_unknown_fields))]
pub struct ApiError {
    pub error: String,
    pub message: String,
}

/*
    CREATE TABLE local_account_txs ON CLUSTER '{cluster}'
(
    account_id            String COMMENT 'The account ID',
    transaction_hash      String COMMENT 'The transaction hash',
    last_block_height     UInt64 COMMENT 'The block height when the account was last updated',
    tx_block_height       UInt64 COMMENT 'The block height when the transaction was included into the blockchain',
    tx_block_timestamp    DateTime64(9, 'UTC') COMMENT 'The block timestamp in UTC when the transaction was included',
    tx_index              UInt32 COMMENT 'The index of the transaction in the block',
    is_signer             Bool COMMENT 'True if the account signed the transaction',
    is_delegated_signer   Bool COMMENT 'True if the account was the signer of the delegated transaction action',
    is_real_signer        Bool COMMENT 'True if the account was the real signer of the transaction (either direct or delegated, excluding relayer signer)',
    is_any_signer         Bool COMMENT 'True if the account was the signer of the delegated transaction action or the signer of the transaction',
    is_predecessor        Bool COMMENT 'True if the account was the predecessor of a receipt',
    is_explicit_refund_to Bool COMMENT 'True if the account was the explicitly set as a refund_to account of an action receipt',
    is_receiver           Bool COMMENT 'True if the account was the receiver of a receipt',
    is_real_receiver      Bool COMMENT 'True if the account was the receiver of a receipt (excluding relayer receiver and gas refunds)',
    is_function_call      Bool COMMENT 'True if the account was the target of a function call action',
    is_action_arg         Bool COMMENT 'True if the account was involved in action arguments',
    is_event_log          Bool COMMENT 'True if the account was involved in JSON event logs',
    is_success            Bool COMMENT 'Whether the transaction execution was successful or not. Pending transactions are considered not successful',

    INDEX tx_block_timestamp_minmax_idx tx_block_timestamp TYPE minmax GRANULARITY 1,
    INDEX tx_block_height_minmax_idx tx_block_height TYPE minmax GRANULARITY 1,

) ENGINE = ReplicatedReplacingMergeTree('/clickhouse/tables/{shard}/default/local_account_txs', '{replica}',
                                        last_block_height)
      PARTITION BY toYYYYMM(tx_block_timestamp)
      PRIMARY KEY (account_id, tx_block_height)
      ORDER BY (account_id, tx_block_height, tx_index)
*/
#[serde_as]
#[derive(Row, Serialize, Deserialize, Clone, Debug)]
#[cfg_attr(feature = "openapi", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "openapi", schemars(deny_unknown_fields))]
pub struct AccountTxRow {
    pub account_id: String,
    pub transaction_hash: String,
    pub tx_block_height: u64,
    #[serde_as(serialize_as = "DisplayFromStr", deserialize_as = "_")]
    #[cfg_attr(feature = "openapi", schemars(with = "String"))]
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

/*
CREATE TABLE local_transactions ON CLUSTER '{cluster}'
(
    transaction_hash   String COMMENT 'Transaction hash',
    signer_id          String COMMENT 'The account ID of the transaction signer',
    tx_block_height    UInt64 COMMENT 'The block height when the transaction was included',
    tx_index           UInt32 COMMENT 'The index of the transaction in the block',
    tx_block_hash      String COMMENT 'The block hash when the transaction was included',
    tx_block_timestamp DateTime64(9, 'UTC') COMMENT 'The block timestamp in UTC when the transaction was included',
    last_block_height  UInt64 COMMENT 'The block height when the last receipt was processed for the transaction',
    is_completed       Bool COMMENT 'Whether the transaction has all the data or still pending some receipts',
    shard_id           UInt64 COMMENT 'The shard ID where the transaction was included',
    receiver_id        String COMMENT 'The account ID of the transaction receiver',
    signer_public_key  String COMMENT 'The public key of the transaction signer',
    priority_fee       UInt64 COMMENT 'The priority fee of the transaction',
    nonce              UInt64 COMMENT 'The nonce of the transaction',
    is_relayed         Bool COMMENT 'Whether the transaction is relayed or not',
    real_signer_id     String COMMENT 'The account ID of the signer of the delegated transaction action, if applicable. Otherwise same as signer_id',
    real_receiver_id   String COMMENT 'The account ID of the receiver of the delegated transaction action, if applicable. Otherwise same as receiver_id',
    is_success         Bool COMMENT 'Whether the transaction execution was successful or not. Pending transactions are considered not successful',
    gas_burnt          UInt64 COMMENT 'The amount of burnt gas for the execution of the whole transaction',
    tokens_burnt       UInt128 COMMENT 'The amount of tokens in yoctoNEAR burnt for the execution of the whole transaction',

    INDEX transaction_hash_bloom_index transaction_hash TYPE bloom_filter() GRANULARITY 1,
    INDEX signer_id_bloom_index signer_id TYPE bloom_filter() GRANULARITY 1,
    INDEX tx_block_height_minmax_idx tx_block_height TYPE minmax GRANULARITY 1,
    INDEX tx_block_timestamp_minmax_idx tx_block_timestamp TYPE minmax GRANULARITY 1,
) ENGINE = ReplicatedReplacingMergeTree('/clickhouse/tables/{shard}/default/local_transactions', '{replica}',
                                        last_block_height)
      PARTITION BY toYYYYMM(tx_block_timestamp)
      PRIMARY KEY (tx_block_height)
      ORDER BY (tx_block_height, tx_index)
 */
#[serde_as]
#[derive(Row, Serialize, Deserialize, Clone, Debug)]
#[cfg_attr(feature = "openapi", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "openapi", schemars(deny_unknown_fields))]
pub struct BlockTxRow {
    pub transaction_hash: String,
    pub signer_id: String,
    pub tx_block_height: u64,
    pub tx_index: u32,
    pub tx_block_hash: String,
    #[serde_as(serialize_as = "DisplayFromStr", deserialize_as = "_")]
    #[cfg_attr(feature = "openapi", schemars(with = "String"))]
    pub tx_block_timestamp: u64,
    pub last_block_height: u64,
    pub is_completed: bool,
    pub shard_id: u64,
    pub receiver_id: String,
    pub signer_public_key: String,
    pub priority_fee: u64,
    pub nonce: u64,
    pub is_relayed: bool,
    pub real_signer_id: String,
    pub real_receiver_id: String,
    pub is_success: bool,
    pub gas_burnt: u64,
    #[serde_as(serialize_as = "DisplayFromStr", deserialize_as = "_")]
    #[cfg_attr(feature = "openapi", schemars(with = "String"))]
    pub tokens_burnt: u128,
}

/*
CREATE TABLE local_receipt_txs ON CLUSTER '{cluster}'
(
    receipt_id           String COMMENT 'The receipt hash',
    block_height         UInt64 COMMENT 'The block height when the receipt was executed',
    block_timestamp      DateTime64(9, 'UTC') COMMENT 'The block timestamp in UTC when the receipt was executed',
    receipt_index        UInt32 COMMENT 'Index of the receipt that was executed in the block across all shards',
    appear_block_height  UInt64 COMMENT 'The block height when the receipt first appeared (e.g. data receipts appear earlier)',
    appear_receipt_index UInt32 COMMENT 'Index of the receipt that first appeared in the block across all shards',
    transaction_hash     String COMMENT 'The transaction hash',
    tx_block_height      UInt64 COMMENT 'The block height when the transaction was included',
    tx_block_timestamp   DateTime64(9, 'UTC') COMMENT 'The block timestamp in UTC when the transaction was included',
    tx_index             UInt32 COMMENT 'The index of the transaction in the block',
    predecessor_id       String COMMENT 'The account ID of the receipt predecessor',
    receiver_id          String COMMENT 'The account ID of where the receipt is executed',
    receipt_type         LowCardinality(String) COMMENT 'The type of the receipt: Action, Data, GlobalContractDistribution',
    priority             UInt64 COMMENT 'The priority of the receipt',
    shard_id             UInt64 COMMENT 'The shard ID where the receipt was executed',
    is_success           Bool COMMENT 'Whether the receipt execution was successful or not',

    INDEX receipt_id_bloom_index receipt_id TYPE bloom_filter() GRANULARITY 1,
    INDEX tx_block_timestamp_minmax_idx tx_block_height TYPE minmax GRANULARITY 1,
) ENGINE = ReplicatedReplacingMergeTree('/clickhouse/tables/{shard}/default/local_receipt_txs', '{replica}')
      PARTITION BY toYYYYMM(block_timestamp)
      PRIMARY KEY (block_height, receipt_index)
      ORDER BY (block_height, receipt_index, receipt_id)

 */
#[serde_as]
#[derive(Row, Serialize, Deserialize, Clone, Debug)]
#[cfg_attr(feature = "openapi", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "openapi", schemars(deny_unknown_fields))]
pub struct ReceiptTxRow {
    pub receipt_id: String,
    pub block_height: u64,
    #[serde_as(serialize_as = "DisplayFromStr", deserialize_as = "_")]
    #[cfg_attr(feature = "openapi", schemars(with = "String"))]
    pub block_timestamp: u64,
    pub receipt_index: u32,
    pub appear_block_height: u64,
    pub appear_receipt_index: u32,
    pub transaction_hash: String,
    pub tx_block_height: u64,
    #[serde_as(serialize_as = "DisplayFromStr", deserialize_as = "_")]
    #[cfg_attr(feature = "openapi", schemars(with = "String"))]
    pub tx_block_timestamp: u64,
    pub tx_index: u32,
    pub predecessor_id: String,
    pub receiver_id: String,
    pub receipt_type: String,
    pub priority: u64,
    pub shard_id: u64,
    pub is_success: bool,
}

/*
CREATE TABLE local_blocks ON CLUSTER '{cluster}'
(
    block_height      UInt64 COMMENT 'The block height',
    prev_block_height Nullable(UInt64) COMMENT 'The previous block height',
    block_hash        String COMMENT 'The block hash',
    prev_block_hash   String COMMENT 'The previous block hash',
    block_timestamp   DateTime64(9, 'UTC') COMMENT 'The block timestamp in UTC',
    epoch_id          String COMMENT 'The epoch ID',
    next_epoch_id     String COMMENT 'The next epoch ID',
    chunks_included   UInt64 COMMENT 'The number of chunks included in the block',
    author_id         String COMMENT 'The account ID of the block author',
    protocol_version  UInt32 COMMENT 'The protocol version',
    gas_price         UInt128 COMMENT 'The gas price in yoctoNEAR',
    block_ordinal     Nullable(UInt64) COMMENT 'The block ordinal in the chain',
    total_supply      UInt128 COMMENT 'The total supply in yoctoNEAR at this block',
    num_transactions  UInt32 COMMENT 'The number of transactions in the block (executed)',
    num_receipts      UInt32 COMMENT 'The number of receipts in the block (executed or used)',
    gas_burnt         UInt64 COMMENT 'The total gas burnt in the block',
    tokens_burnt      UInt128 COMMENT 'The total tokens burnt in yoctoNEAR in the block',

    INDEX block_timestamp_minmax_idx block_timestamp TYPE minmax GRANULARITY 1,
    INDEX author_id_bloom_index author_id TYPE bloom_filter() GRANULARITY 1,
    INDEX epoch_id_bloom_index epoch_id TYPE bloom_filter() GRANULARITY 1,
    INDEX block_hash_bloom_index block_hash TYPE bloom_filter() GRANULARITY 1,
    INDEX protocol_version_minmax_idx protocol_version TYPE minmax GRANULARITY 1,
) ENGINE = ReplicatedReplacingMergeTree('/clickhouse/tables/{shard}/default/local_blocks', '{replica}')
      PARTITION BY toYYYYMM(block_timestamp)
      PRIMARY KEY (block_height)
      ORDER BY (block_height)
 */
#[serde_as]
#[derive(Row, Serialize, Deserialize, Clone, Debug)]
#[cfg_attr(feature = "openapi", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "openapi", schemars(deny_unknown_fields))]
pub struct BlockRow {
    pub block_height: u64,
    pub prev_block_height: Option<u64>,
    pub block_hash: String,
    pub prev_block_hash: String,
    #[serde_as(serialize_as = "DisplayFromStr", deserialize_as = "_")]
    #[cfg_attr(feature = "openapi", schemars(with = "String"))]
    pub block_timestamp: u64,
    pub epoch_id: String,
    pub next_epoch_id: String,
    pub chunks_included: u64,
    pub author_id: String,
    pub protocol_version: u32,
    #[serde_as(serialize_as = "DisplayFromStr", deserialize_as = "_")]
    #[cfg_attr(feature = "openapi", schemars(with = "String"))]
    pub gas_price: u128,
    pub block_ordinal: Option<u64>,
    #[serde_as(serialize_as = "DisplayFromStr", deserialize_as = "_")]
    #[cfg_attr(feature = "openapi", schemars(with = "String"))]
    pub total_supply: u128,
    pub num_transactions: u32,
    pub num_receipts: u32,
    #[serde_as(serialize_as = "DisplayFromStr", deserialize_as = "_")]
    #[cfg_attr(feature = "openapi", schemars(with = "String"))]
    pub gas_burnt: u64,
    #[serde_as(serialize_as = "DisplayFromStr", deserialize_as = "_")]
    #[cfg_attr(feature = "openapi", schemars(with = "String"))]
    pub tokens_burnt: u128,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum BlockId {
    Hash(CryptoHash),
    Height(BlockHeight),
}

#[cfg(feature = "openapi")]
fn tx_hashes_schema(_generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
    schemars::json_schema!({
        "type": "array",
        "items": {
            "type": "string"
        },
        "maxItems": 20
    })
}

#[cfg(feature = "openapi")]
fn raw_transaction_schema(_generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
    schemars::json_schema!({
        "type": "object",
        "additionalProperties": true
    })
}

#[cfg(feature = "openapi")]
fn raw_transaction_array_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
    let mut schema = schemars::json_schema!({
        "type": "array"
    });
    schema
        .ensure_object()
        .insert("items".into(), raw_transaction_schema(generator).to_value());
    schema
}

#[cfg(feature = "openapi")]
fn nullable_raw_transaction_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
    let mut schema = raw_transaction_schema(generator);
    schema
        .ensure_object()
        .insert("nullable".into(), true.into());
    schema
}

#[cfg(feature = "openapi")]
fn block_id_schema(_generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
    schemars::json_schema!({
        "oneOf": [
            {
                "type": "integer",
                "minimum": 0
            },
            {
                "type": "string"
            }
        ]
    })
}

#[cfg(feature = "openapi")]
fn string_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
    <String as schemars::JsonSchema>::json_schema(generator)
}

#[cfg(feature = "openapi")]
fn u64_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
    <u64 as schemars::JsonSchema>::json_schema(generator)
}

#[cfg(feature = "openapi")]
fn block_txs_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
    <Vec<BlockTxRow> as schemars::JsonSchema>::json_schema(generator)
}

#[cfg(feature = "openapi")]
fn receipt_txs_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
    <Vec<ReceiptTxRow> as schemars::JsonSchema>::json_schema(generator)
}

#[cfg(feature = "openapi")]
fn nullable_block_row_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
    let mut schema = <BlockRow as schemars::JsonSchema>::json_schema(generator);
    schema
        .ensure_object()
        .insert("nullable".into(), true.into());
    schema
}

#[cfg(feature = "openapi")]
fn nullable_receipt_tx_row_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
    let mut schema = <ReceiptTxRow as schemars::JsonSchema>::json_schema(generator);
    schema
        .ensure_object()
        .insert("nullable".into(), true.into());
    schema
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use serde_json::value::RawValue;

    use super::{AccountResponse, ReceiptResponse, TransactionsResponse};

    fn raw(json_source: &str) -> Box<RawValue> {
        RawValue::from_string(json_source.to_string()).unwrap()
    }

    #[test]
    fn account_response_omits_optional_fields_when_absent() {
        let response = AccountResponse {
            account_txs: vec![],
            resume_token: None,
            txs_count: None,
        };

        assert_eq!(
            serde_json::to_value(response).unwrap(),
            json!({ "account_txs": [] })
        );
    }

    #[test]
    fn transactions_and_receipt_responses_preserve_raw_json_payloads() {
        let transactions = TransactionsResponse {
            transactions: vec![raw(r#"{"transaction_hash":"abc","nested":{"ok":true}}"#)],
        };
        let receipt = ReceiptResponse {
            receipt: None,
            transaction: Some(raw(r#"{"transaction_hash":"abc","nested":{"ok":true}}"#)),
        };

        assert_eq!(
            serde_json::to_value(transactions).unwrap(),
            json!({
                "transactions": [
                    {
                        "transaction_hash": "abc",
                        "nested": { "ok": true }
                    }
                ]
            })
        );
        assert_eq!(
            serde_json::to_value(receipt).unwrap(),
            json!({
                "receipt": null,
                "transaction": {
                    "transaction_hash": "abc",
                    "nested": { "ok": true }
                }
            })
        );
    }
}
