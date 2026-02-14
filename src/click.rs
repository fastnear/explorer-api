use crate::*;

use clickhouse::{Client, Row};
use serde::de::DeserializeOwned;
use serde_json::value::RawValue;
use std::env;

const ACCOUNT_DEFAULT_LIMIT: usize = 200;

#[derive(Clone)]
pub struct ClickDB {
    pub client: Client,
}

impl ClickDB {
    pub fn new() -> Self {
        Self {
            client: establish_connection(),
        }
    }

    pub async fn max(&self, column: &str, table: &str) -> clickhouse::error::Result<BlockHeight> {
        let block_height = self
            .client
            .query(&format!("SELECT max({}) FROM {}", column, table))
            .fetch_one::<u64>()
            .await?;
        Ok(block_height)
    }

    pub async fn verify_connection(&self) -> clickhouse::error::Result<()> {
        self.client.query("SELECT 1").execute().await?;
        Ok(())
    }

    pub async fn read_rows<T>(&self, query: &str) -> clickhouse::error::Result<Vec<T>>
    where
        T: Row + DeserializeOwned,
    {
        let rows = self.client.query(query).fetch_all::<T>().await?;
        Ok(rows)
    }

    pub async fn get_transactions(
        &self,
        tx_hashes: &[String],
    ) -> clickhouse::error::Result<Vec<Box<RawValue>>> {
        if tx_hashes.is_empty() {
            return Ok(vec![]);
        }
        let query = format!(
            "select data from raw_tx where transaction_hash in ({})",
            tx_hashes
                .iter()
                .map(|h| format!("'{}'", h))
                .collect::<Vec<_>>()
                .join(",")
        );
        Ok(self
            .client
            .query(&query)
            .fetch_all::<Vec<u8>>()
            .await?
            .into_iter()
            .map(|v| {
                RawValue::from_string(String::from_utf8(zstd::decode_all(&v[..]).unwrap()).unwrap())
                    .unwrap()
            })
            .collect())
    }

    pub async fn get_account_txs(
        &self,
        account_id: &AccountId,
        max_block_height: Option<BlockHeight>,
    ) -> clickhouse::error::Result<Vec<AccountTxRow>> {
        let max_block_height = max_block_height.unwrap_or(u64::MAX);
        let query = format!(
            "select account_id, transaction_hash, tx_block_height, tx_block_timestamp, tx_index from account_txs where account_id = '{}' and tx_block_height <= {} order by tx_block_height desc, tx_index desc limit {}",
            account_id,
            max_block_height,
            ACCOUNT_DEFAULT_LIMIT
        );
        self.read_rows(&query).await
    }

    pub async fn get_account_txs_count(
        &self,
        account_id: &AccountId,
    ) -> clickhouse::error::Result<u64> {
        let query = format!(
            "select count() from account_txs where account_id = '{}'",
            account_id
        );
        let count = self.client.query(&query).fetch_one::<u64>().await?;
        Ok(count)
    }

    pub async fn get_block_txs(
        &self,
        block_id: BlockId,
    ) -> clickhouse::error::Result<Vec<BlockTxRow>> {
        let query = match block_id {
            BlockId::Hash(hash) => {
                format!(
                    "WITH block_heights AS (
                        SELECT DISTINCT block_height
                        FROM block_txs
                        WHERE block_hash = '{}'
                        LIMIT 1
                    )
                    SELECT * FROM block_txs WHERE block_height IN block_heights",
                    hash.to_string()
                )
            }
            BlockId::Height(height) => {
                format!("SELECT * FROM block_txs WHERE block_height = {}", height)
            }
        };
        self.read_rows(&query).await
    }

    #[allow(dead_code)]
    pub async fn get_last_block_txs(
        &self,
        limit: usize,
    ) -> clickhouse::error::Result<Vec<BlockTxRow>> {
        let query = format!(
            "WITH last_blocks AS (
                SELECT DISTINCT block_height
                FROM block_txs
                ORDER BY block_height DESC
                LIMIT {}
            )
            SELECT
                *
            FROM
                block_txs
            WHERE
                block_height in last_blocks",
            limit
        );
        self.read_rows(&query).await
    }

    pub async fn get_last_blocks(&self, limit: usize) -> clickhouse::error::Result<Vec<BlockRow>> {
        let query = format!(
            "
            SELECT
                block_height,
                prev_block_height,
                block_hash,
                prev_block_hash,
                block_timestamp,
                epoch_id,
                next_epoch_id,
                chunks_included,
                author_id,
                protocol_version,
                gas_price,
                block_ordinal,
                total_supply,
                num_transactions,
                num_receipts,
                gas_burnt,
                tokens_burnt
            FROM
                blocks
            ORDER BY
                block_height DESC
            LIMIT {}",
            limit
        );
        self.read_rows(&query).await
    }

    pub async fn get_tx_for_receipt(
        &self,
        receipt_id: &str,
    ) -> clickhouse::error::Result<Option<Box<RawValue>>> {
        let query = format!(
            "WITH last_txs AS (
                SELECT
                    transaction_hash
                FROM
                    receipt_txs
                WHERE
                    receipt_id = '{}'
                LIMIT 1
            )
            SELECT
                transaction
            FROM
                transactions
            WHERE
                transaction_hash in last_txs",
            receipt_id
        );
        let transactions = self.client.query(&query).fetch_all::<String>().await?;
        Ok(transactions
            .into_iter()
            .next()
            .map(|s| RawValue::from_string(s).unwrap()))
    }
}

fn establish_connection() -> Client {
    Client::default()
        .with_url(env::var("DATABASE_URL").unwrap())
        .with_user(env::var("DATABASE_USER").unwrap())
        .with_password(env::var("DATABASE_PASSWORD").unwrap())
        .with_database(env::var("DATABASE_DATABASE").unwrap())
}
