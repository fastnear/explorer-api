use std::fmt;
use std::time::{SystemTime, UNIX_EPOCH};

use actix_web::{post, ResponseError};
use actix_web::{web, HttpRequest, HttpResponse, Responder};

use crate::types::{
    AccountInput, AccountResponse, ApiError, BlockInput, BlockResponse, BlocksInput,
    BlocksResponse, HealthResponse, ReceiptInput, ReceiptResponse, StatusResponse,
    TransactionsResponse, TxInput,
};
use crate::AppState;

const TARGET_API: &str = "api";
const DEFAULT_TX_LIMIT: usize = 20;
const ACCOUNT_DEFAULT_LIMIT: usize = 200;
const MAX_BLOCK_HEIGHT: u64 = 1_000_000_000_000_000;

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
                HttpResponse::BadRequest().json(ApiError {
                    error: "Bad request".to_string(),
                    message: err.clone(),
                })
            }
        }
    }
}

/// Default staleness budget for `/health`, in seconds. NEAR produces a block roughly
/// every 1.2s, so this trips only when the indexer has genuinely stopped.
pub const DEFAULT_MAX_LAG_SECONDS: u64 = 60;

/// Why the service is not able to serve fresh data.
#[derive(Debug)]
enum Unhealthy {
    Clickhouse(clickhouse::error::Error),
    NoBlocks,
    Stale {
        lag_seconds: u64,
        max_lag_seconds: u64,
    },
}

/// The numbers `/health` and `/status` share. Both endpoints run the same probe and
/// differ only in how much of it they print.
struct Probe {
    latest_block_height: u64,
    latest_block_timestamp: u64,
    latest_tx_block_height: u64,
    lag_seconds: u64,
}

fn now_nanos() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0)
}

/// Saturating so that a block timestamp marginally ahead of the local clock reports 0
/// rather than underflowing.
fn compute_lag_seconds(block_timestamp_nanos: u64, now_nanos: u128) -> u64 {
    (now_nanos.saturating_sub(block_timestamp_nanos as u128) / 1_000_000_000) as u64
}

/// A successful read of `blocks_latest` proves the connection is up, so there is no
/// separate `SELECT 1` ping on top of it.
async fn probe(app_state: &AppState) -> Result<Probe, Unhealthy> {
    let (blocks, latest_tx_block_height) = tokio::try_join!(
        app_state.click_db.get_blocks(None, None, 1, true),
        app_state
            .click_db
            .latest_height("tx_block_height", "transactions"),
    )
    .map_err(Unhealthy::Clickhouse)?;

    let block = blocks.into_iter().next().ok_or(Unhealthy::NoBlocks)?;
    let latest_tx_block_height = latest_tx_block_height.ok_or(Unhealthy::NoBlocks)?;

    let lag_seconds = compute_lag_seconds(block.block_timestamp, now_nanos());
    if lag_seconds > app_state.max_lag_seconds {
        return Err(Unhealthy::Stale {
            lag_seconds,
            max_lag_seconds: app_state.max_lag_seconds,
        });
    }

    Ok(Probe {
        latest_block_height: block.block_height,
        latest_block_timestamp: block.block_timestamp,
        latest_tx_block_height,
        lag_seconds,
    })
}

impl Unhealthy {
    /// The 503 body. Kept free of side effects so it can be unit-tested without a
    /// database; the Clickhouse error itself is logged by `unhealthy_response`.
    fn body(&self) -> HealthResponse {
        let (message, lag_seconds) = match self {
            // The underlying error is logged, not returned, so database internals are
            // not leaked to callers.
            Unhealthy::Clickhouse(_) => ("Clickhouse is unreachable".to_string(), None),
            Unhealthy::NoBlocks => ("No indexed blocks".to_string(), None),
            Unhealthy::Stale {
                lag_seconds,
                max_lag_seconds,
            } => (
                format!(
                    "Indexer is stale: latest block is {}s behind (max {}s)",
                    lag_seconds, max_lag_seconds
                ),
                Some(*lag_seconds),
            ),
        };

        HealthResponse {
            status: "unhealthy".to_string(),
            lag_seconds,
            message: Some(message),
        }
    }
}

/// Both endpoints render the same 503 body, so they can never disagree.
fn unhealthy_response(reason: Unhealthy) -> HttpResponse {
    if let Unhealthy::Clickhouse(err) = &reason {
        tracing::error!(target: TARGET_API, "Health check Clickhouse error: {:?}", err);
    }
    HttpResponse::ServiceUnavailable().json(reason.body())
}

pub async fn health(app_state: web::Data<AppState>) -> impl Responder {
    match probe(&app_state).await {
        Ok(probe) => HttpResponse::Ok().json(HealthResponse {
            status: "ok".to_string(),
            lag_seconds: Some(probe.lag_seconds),
            message: None,
        }),
        Err(reason) => unhealthy_response(reason),
    }
}

pub async fn status(app_state: web::Data<AppState>) -> impl Responder {
    match probe(&app_state).await {
        Ok(probe) => HttpResponse::Ok().json(StatusResponse {
            status: "ok".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            uptime_seconds: app_state.started_at.elapsed().as_secs(),
            latest_block_height: probe.latest_block_height,
            latest_block_timestamp: probe.latest_block_timestamp,
            lag_seconds: probe.lag_seconds,
            latest_tx_block_height: probe.latest_tx_block_height,
        }),
        Err(reason) => unhealthy_response(reason),
    }
}

pub mod v0 {
    use super::*;

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
        Ok(web::Json(TransactionsResponse { transactions }))
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
        if let Some(v) = is_signer {
            bool_filters.push(("is_signer", v));
        }
        if let Some(v) = is_delegated_signer {
            bool_filters.push(("is_delegated_signer", v));
        }
        if let Some(v) = is_real_signer {
            bool_filters.push(("is_real_signer", v));
        }
        if let Some(v) = is_any_signer {
            bool_filters.push(("is_any_signer", v));
        }
        if let Some(v) = is_predecessor {
            bool_filters.push(("is_predecessor", v));
        }
        if let Some(v) = is_explicit_refund_to {
            bool_filters.push(("is_explicit_refund_to", v));
        }
        if let Some(v) = is_receiver {
            bool_filters.push(("is_receiver", v));
        }
        if let Some(v) = is_real_receiver {
            bool_filters.push(("is_real_receiver", v));
        }
        if let Some(v) = is_function_call {
            bool_filters.push(("is_function_call", v));
        }
        if let Some(v) = is_action_arg {
            bool_filters.push(("is_action_arg", v));
        }
        if let Some(v) = is_event_log {
            bool_filters.push(("is_event_log", v));
        }
        if let Some(v) = is_success {
            bool_filters.push(("is_success", v));
        }

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

        Ok(web::Json(AccountResponse {
            account_txs,
            resume_token,
            txs_count,
        }))
    }

    #[post("/block")]
    pub async fn get_block(
        _request: HttpRequest,
        input: web::Json<BlockInput>,
        app_state: web::Data<AppState>,
    ) -> Result<impl Responder, ServiceError> {
        let BlockInput {
            block_id,
            with_transactions,
            with_receipts,
        } = input.into_inner();
        let with_transactions = with_transactions.unwrap_or(false);
        let with_receipts = with_receipts.unwrap_or(false);

        let block = app_state.click_db.get_block(block_id).await?;
        let mut block_txs = None;
        let mut block_receipts = None;

        if let Some(ref block) = block {
            let height = block.block_height;
            match (with_transactions, with_receipts) {
                (true, true) => {
                    let (transactions, receipts) = tokio::try_join!(
                        app_state.click_db.get_block_txs(height),
                        app_state.click_db.get_block_receipts(height),
                    )?;
                    block_txs = Some(transactions);
                    block_receipts = Some(receipts);
                }
                (true, false) => {
                    block_txs = Some(app_state.click_db.get_block_txs(height).await?);
                }
                (false, true) => {
                    block_receipts = Some(app_state.click_db.get_block_receipts(height).await?);
                }
                _ => {}
            }
        }

        Ok(web::Json(BlockResponse {
            block,
            block_txs,
            block_receipts,
        }))
    }

    const BLOCKS_DEFAULT_LIMIT: usize = 100;

    #[post("/blocks")]
    pub async fn get_blocks(
        _request: HttpRequest,
        input: web::Json<BlocksInput>,
        app_state: web::Data<AppState>,
    ) -> Result<impl Responder, ServiceError> {
        let BlocksInput {
            from_block_height,
            to_block_height,
            limit,
            desc,
        } = input.into_inner();

        let desc = desc.unwrap_or(true);
        let limit = limit
            .unwrap_or(BLOCKS_DEFAULT_LIMIT)
            .min(BLOCKS_DEFAULT_LIMIT);

        if from_block_height.unwrap_or(0) > MAX_BLOCK_HEIGHT
            || to_block_height.unwrap_or(0) > MAX_BLOCK_HEIGHT
        {
            return Err(ServiceError::ArgumentError(format!(
                "Block height exceeds maximum allowed value of {}",
                MAX_BLOCK_HEIGHT
            )));
        }

        let blocks = app_state
            .click_db
            .get_blocks(from_block_height, to_block_height, limit, desc)
            .await?;

        Ok(web::Json(BlocksResponse { blocks }))
    }

    #[post("/receipt")]
    pub async fn get_receipt(
        _request: HttpRequest,
        input: web::Json<ReceiptInput>,
        app_state: web::Data<AppState>,
    ) -> Result<impl Responder, ServiceError> {
        let ReceiptInput { receipt_id } = input.into_inner();
        let receipt = app_state
            .click_db
            .get_receipt(receipt_id.to_string().as_str())
            .await?;
        let transaction = if let Some(ref receipt) = receipt {
            let txs = app_state
                .click_db
                .get_transactions(&[receipt.transaction_hash.clone()])
                .await?;
            txs.into_iter().next()
        } else {
            None
        };
        Ok(web::Json(ReceiptResponse {
            receipt,
            transaction,
        }))
    }
}

#[cfg(test)]
mod tests {
    use actix_web::http::StatusCode;

    use super::{compute_lag_seconds, unhealthy_response, Unhealthy};

    const NANOS_PER_SEC: u128 = 1_000_000_000;

    #[test]
    fn lag_is_whole_seconds_behind_the_clock() {
        let block = 1_700_000_000u64 * NANOS_PER_SEC as u64;
        let now = block as u128 + 42 * NANOS_PER_SEC + 500_000_000;
        assert_eq!(compute_lag_seconds(block, now), 42);
    }

    #[test]
    fn lag_is_zero_when_the_block_is_ahead_of_the_clock() {
        let block = 1_700_000_000u64 * NANOS_PER_SEC as u64;
        let now = block as u128 - 5 * NANOS_PER_SEC;
        assert_eq!(compute_lag_seconds(block, now), 0);
    }

    #[test]
    fn stale_body_reports_the_lag_and_the_threshold() {
        let body = Unhealthy::Stale {
            lag_seconds: 120,
            max_lag_seconds: 60,
        }
        .body();

        assert_eq!(body.status, "unhealthy");
        assert_eq!(body.lag_seconds, Some(120));
        let message = body.message.unwrap();
        assert!(message.contains("120s"), "{}", message);
        assert!(message.contains("60s"), "{}", message);
    }

    #[test]
    fn no_blocks_body_has_no_lag_to_report() {
        let body = Unhealthy::NoBlocks.body();

        assert_eq!(body.status, "unhealthy");
        assert_eq!(body.lag_seconds, None);
        assert_eq!(body.message.unwrap(), "No indexed blocks");
    }

    #[test]
    fn unhealthy_reasons_render_503() {
        assert_eq!(
            unhealthy_response(Unhealthy::NoBlocks).status(),
            StatusCode::SERVICE_UNAVAILABLE
        );
        assert_eq!(
            unhealthy_response(Unhealthy::Stale {
                lag_seconds: 120,
                max_lag_seconds: 60,
            })
            .status(),
            StatusCode::SERVICE_UNAVAILABLE
        );
    }
}
