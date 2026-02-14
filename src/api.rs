use crate::*;
use std::fmt;

use actix_web::{post, ResponseError};
use actix_web::{web, HttpRequest};
use serde::Deserialize;
use serde_with::{serde_as, DisplayFromStr};

use serde_json::json;

const TARGET_API: &str = "api";
const DEFAULT_TX_LIMIT: usize = 20;
const ACCOUNT_DEFAULT_LIMIT: usize = 200;
const MAX_BLOCK_HEIGHT: BlockHeight = 1_000_000_000_000_000;

#[derive(Debug)]
enum ServiceError {
    ClickhouseError(clickhouse::error::Error),
    ArgumentError(String),
}

impl From<clickhouse::error::Error> for ServiceError {
    fn from(error: clickhouse::error::Error) -> Self {
        ServiceError::ClickhouseError(error)
    }
}

impl fmt::Display for ServiceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ServiceError::ClickhouseError(ref err) => write!(f, "Database Error: {:?}", err),
            ServiceError::ArgumentError(ref err) => write!(f, "Argument Error: {}", err),
        }
    }
}

impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            ServiceError::ClickhouseError(ref err) => {
                tracing::error!(target: TARGET_API, "Clickhouse error: {:?}", err);
                HttpResponse::InternalServerError().json("Internal server error (Clickhouse)")
            }
            ServiceError::ArgumentError(ref err) => {
                tracing::error!(target: TARGET_API, "Argument error: {}", err);
                HttpResponse::BadRequest().json(json!({
                    "error": "Bad request",
                    "message": err,
                }))
            }
        }
    }
}

pub mod v0 {
    use super::*;

    #[derive(Debug, Deserialize)]
    pub struct TxInput {
        pub tx_hashes: Vec<CryptoHash>,
    }

    #[post("/transactions")]
    pub async fn get_transactions(
        _request: HttpRequest,
        input: web::Json<TxInput>,
        app_state: web::Data<AppState>,
    ) -> Result<impl Responder, ServiceError> {
        let TxInput { tx_hashes } = input.into_inner();
        if tx_hashes.len() > DEFAULT_TX_LIMIT {
            return Err(ServiceError::ArgumentError(format!(
                "Too many transactions requested. Limit is {}",
                DEFAULT_TX_LIMIT
            )));
        }

        let tx_hashes = tx_hashes.iter().map(|h| h.to_string()).collect::<Vec<_>>();
        let transactions = app_state.click_db.get_transactions(&tx_hashes).await?;
        Ok(web::Json(json!({
            "transactions": transactions,
        })))
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
    #[derive(Debug, Deserialize)]
    pub struct AccountInput {
        pub account_id: AccountId,
        pub is_signer: Option<bool>,
        pub is_delegated_signer: Option<bool>,
        pub is_real_signer: Option<bool>,
        pub is_any_signer: Option<bool>,
        pub is_predecessor: Option<bool>,
        pub is_explicit_refund_to: Option<bool>,
        pub is_receiver: Option<bool>,
        pub is_real_receiver: Option<bool>,
        pub is_function_call: Option<bool>,
        pub is_action_arg: Option<bool>,
        pub is_event_log: Option<bool>,
        pub is_success: Option<bool>,
        #[serde_as(as = "Option<DisplayFromStr>")]
        pub resume_token: Option<u128>,
        pub from_tx_block_height: Option<BlockHeight>,
        pub to_tx_block_height: Option<BlockHeight>,
        pub limit: Option<usize>,
        pub desc: Option<bool>,
    }

    #[post("/account")]
    pub async fn get_account(
        _request: HttpRequest,
        input: web::Json<AccountInput>,
        app_state: web::Data<AppState>,
    ) -> Result<impl Responder, ServiceError> {
        let AccountInput {
            account_id,
            is_signer,
            is_delegated_signer,
            is_real_signer,
            is_any_signer,
            is_predecessor,
            is_explicit_refund_to,
            is_receiver,
            is_real_receiver,
            is_function_call,
            is_action_arg,
            is_event_log,
            is_success,
            resume_token,
            from_tx_block_height,
            to_tx_block_height,
            limit,
            desc,
        } = input.into_inner();

        let desc = desc.unwrap_or(true);
        let limit = limit
            .unwrap_or(ACCOUNT_DEFAULT_LIMIT)
            .min(ACCOUNT_DEFAULT_LIMIT);

        if from_tx_block_height.unwrap_or(0) > MAX_BLOCK_HEIGHT
            || to_tx_block_height.unwrap_or(0) > MAX_BLOCK_HEIGHT
        {
            return Err(ServiceError::ArgumentError(format!(
                "Block height exceeds maximum allowed value of {}",
                MAX_BLOCK_HEIGHT
            )));
        }

        // Parse resume token into (tx_block_height, tx_index)
        let resume = resume_token.map(|token| {
            let block_height = (token >> 32).min(MAX_BLOCK_HEIGHT as u128) as u64;
            let tx_index = (token & 0xFFFFFFFF) as u32;
            (block_height, tx_index)
        });

        // Build boolean filters
        let mut bool_filters: Vec<(&str, bool)> = Vec::new();
        if let Some(v) = is_signer { bool_filters.push(("is_signer", v)); }
        if let Some(v) = is_delegated_signer { bool_filters.push(("is_delegated_signer", v)); }
        if let Some(v) = is_real_signer { bool_filters.push(("is_real_signer", v)); }
        if let Some(v) = is_any_signer { bool_filters.push(("is_any_signer", v)); }
        if let Some(v) = is_predecessor { bool_filters.push(("is_predecessor", v)); }
        if let Some(v) = is_explicit_refund_to { bool_filters.push(("is_explicit_refund_to", v)); }
        if let Some(v) = is_receiver { bool_filters.push(("is_receiver", v)); }
        if let Some(v) = is_real_receiver { bool_filters.push(("is_real_receiver", v)); }
        if let Some(v) = is_function_call { bool_filters.push(("is_function_call", v)); }
        if let Some(v) = is_action_arg { bool_filters.push(("is_action_arg", v)); }
        if let Some(v) = is_event_log { bool_filters.push(("is_event_log", v)); }
        if let Some(v) = is_success { bool_filters.push(("is_success", v)); }

        let txs_count = if resume_token.is_none() {
            Some(
                app_state
                    .click_db
                    .get_account_txs_count(
                        &account_id,
                        &bool_filters,
                        from_tx_block_height,
                        to_tx_block_height,
                    )
                    .await?,
            )
        } else {
            None
        };

        let account_txs = app_state
            .click_db
            .get_account_txs(
                &account_id,
                &bool_filters,
                resume,
                from_tx_block_height,
                to_tx_block_height,
                limit,
                desc,
            )
            .await?;

        let resume_token = if account_txs.len() == limit {
            let last = account_txs.last().unwrap();
            let token = (last.tx_block_height as u128) << 32 | (last.tx_index as u128);
            Some(token.to_string())
        } else {
            None
        };

        let mut res = json!({
            "account_txs": account_txs,
        });
        if let Some(resume_token) = resume_token {
            res["resume_token"] = json!(resume_token);
        }
        if let Some(txs_count) = txs_count {
            res["txs_count"] = json!(txs_count);
        }
        Ok(web::Json(res))
    }

    #[derive(Debug, Deserialize)]
    pub struct BlockInput {
        pub block_id: BlockId,
    }

    #[post("/block")]
    pub async fn get_block(
        _request: HttpRequest,
        input: web::Json<BlockInput>,
        app_state: web::Data<AppState>,
    ) -> Result<impl Responder, ServiceError> {
        let BlockInput { block_id } = input.into_inner();
        let block_txs = app_state.click_db.get_block_txs(block_id).await?;
        let tx_hashes = block_txs
            .iter()
            .map(|row| row.transaction_hash.clone())
            .collect::<Vec<_>>();

        let transactions = app_state
            .click_db
            .get_transactions(&tx_hashes[..tx_hashes.len().min(DEFAULT_TX_LIMIT)])
            .await?;
        Ok(web::Json(json!({
            "transactions": transactions,
            "block_txs": block_txs,
        })))
    }

    #[post("/blocks/last")]
    pub async fn get_last_blocks(
        _request: HttpRequest,
        app_state: web::Data<AppState>,
    ) -> Result<impl Responder, ServiceError> {
        let blocks = app_state.click_db.get_last_blocks(10).await?;

        Ok(web::Json(json!({
            "blocks": blocks,
        })))
    }

    #[derive(Debug, Deserialize)]
    pub struct ReceiptInput {
        pub receipt_id: CryptoHash,
    }

    #[post("/receipt")]
    pub async fn get_receipt(
        _request: HttpRequest,
        input: web::Json<ReceiptInput>,
        app_state: web::Data<AppState>,
    ) -> Result<impl Responder, ServiceError> {
        let ReceiptInput { receipt_id } = input.into_inner();
        let transaction = app_state
            .click_db
            .get_tx_for_receipt(receipt_id.to_string().as_str())
            .await?;
        Ok(web::Json(json!({
            "transaction": transaction,
        })))
    }
}
