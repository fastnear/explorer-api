use crate::*;
use std::fmt;

use actix_web::{post, ResponseError};
use actix_web::{web, HttpRequest};
use serde::Deserialize;

use serde_json::json;

const TARGET_API: &str = "api";
const DEFAULT_TX_LIMIT: usize = 20;

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

    #[derive(Debug, Deserialize)]
    pub struct AccountInput {
        pub account_id: AccountId,
        pub max_block_height: Option<BlockHeight>,
    }

    #[post("/account")]
    pub async fn get_account(
        _request: HttpRequest,
        input: web::Json<AccountInput>,
        app_state: web::Data<AppState>,
    ) -> Result<impl Responder, ServiceError> {
        let AccountInput {
            account_id,
            max_block_height,
        } = input.into_inner();
        let txs_count = if max_block_height.is_none() {
            Some(
                app_state
                    .click_db
                    .get_account_txs_count(&account_id)
                    .await?,
            )
        } else {
            None
        };
        let account_txs = app_state
            .click_db
            .get_account_txs(&account_id, max_block_height)
            .await?;
        let tx_hashes = account_txs
            .iter()
            .map(|row| row.transaction_hash.clone())
            .collect::<Vec<_>>();

        let transactions = app_state
            .click_db
            .get_transactions(&tx_hashes[..tx_hashes.len().min(DEFAULT_TX_LIMIT)])
            .await?;
        let mut res = json!({
            "transactions": transactions,
            "account_txs": account_txs,
        });
        if let Some(txs_count) = txs_count {
            res["txs_count"] = json!(txs_count);
        }
        Ok(web::Json(res))
    }

    #[derive(Debug, Deserialize)]
    pub struct BlockInput {
        pub block_height: BlockHeight,
    }

    #[post("/block")]
    pub async fn get_block(
        _request: HttpRequest,
        input: web::Json<BlockInput>,
        app_state: web::Data<AppState>,
    ) -> Result<impl Responder, ServiceError> {
        let BlockInput { block_height } = input.into_inner();
        let block_txs = app_state.click_db.get_block_txs(block_height).await?;
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
        let block_txs = app_state.click_db.get_last_block_txs(5).await?;

        Ok(web::Json(json!({
            "block_txs": block_txs,
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
