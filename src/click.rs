use crate::*;

use clickhouse::{Client, Row};
use serde::de::DeserializeOwned;
use serde_json::value::RawValue;
use std::env;

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

    fn account_txs_conditions(
        account_id: &AccountId,
        bool_filters: &[(&str, bool)],
        from_tx_block_height: Option<BlockHeight>,
        to_tx_block_height: Option<BlockHeight>,
    ) -> Vec<String> {
        let mut conditions = vec![format!("account_id = '{}'", account_id)];

        for (col, val) in bool_filters {
            conditions.push(format!("{} = {}", col, if *val { 1 } else { 0 }));
        }

        if let Some(from) = from_tx_block_height {
            conditions.push(format!("tx_block_height >= {}", from));
        }
        if let Some(to) = to_tx_block_height {
            conditions.push(format!("tx_block_height <= {}", to));
        }

        conditions
    }

    pub async fn get_account_txs(
        &self,
        account_id: &AccountId,
        bool_filters: &[(&str, bool)],
        resume: Option<(u64, u32)>,
        from_tx_block_height: Option<BlockHeight>,
        to_tx_block_height: Option<BlockHeight>,
        limit: usize,
        desc: bool,
    ) -> clickhouse::error::Result<Vec<AccountTxRow>> {
        let mut conditions = Self::account_txs_conditions(
            account_id,
            bool_filters,
            from_tx_block_height,
            to_tx_block_height,
        );

        if let Some((bh, ti)) = resume {
            if desc {
                conditions.push(format!("(tx_block_height, tx_index) < ({}, {})", bh, ti));
            } else {
                conditions.push(format!("(tx_block_height, tx_index) > ({}, {})", bh, ti));
            }
        }

        let order = if desc { "DESC" } else { "ASC" };
        let fetch_limit = limit + 50;
        let query = format!(
            "SELECT account_id, transaction_hash, tx_block_height, tx_block_timestamp, tx_index, \
             is_signer, is_delegated_signer, is_real_signer, is_any_signer, \
             is_predecessor, is_explicit_refund_to, is_receiver, is_real_receiver, \
             is_function_call, is_action_arg, is_event_log, is_success \
             FROM account_txs \
             WHERE {} \
             ORDER BY tx_block_height {}, tx_index {} \
             LIMIT {}",
            conditions.join(" AND "),
            order,
            order,
            fetch_limit
        );
        let mut rows: Vec<AccountTxRow> = self.read_rows(&query).await?;
        rows.dedup_by_key(|r| (r.tx_block_height, r.tx_index));
        rows.truncate(limit);
        Ok(rows)
    }

    pub async fn get_account_txs_count(
        &self,
        account_id: &AccountId,
        bool_filters: &[(&str, bool)],
        from_tx_block_height: Option<BlockHeight>,
        to_tx_block_height: Option<BlockHeight>,
    ) -> clickhouse::error::Result<u64> {
        let conditions = Self::account_txs_conditions(
            account_id,
            bool_filters,
            from_tx_block_height,
            to_tx_block_height,
        );

        let query = format!(
            "SELECT count() FROM account_txs WHERE {}",
            conditions.join(" AND ")
        );
        let count = self.client.query(&query).fetch_one::<u64>().await?;
        Ok(count)
    }

    pub async fn get_block(
        &self,
        block_id: BlockId,
    ) -> clickhouse::error::Result<Option<BlockRow>> {
        let query = match block_id {
            BlockId::Hash(hash) => {
                format!(
                    "SELECT \
                        block_height, prev_block_height, block_hash, prev_block_hash, \
                        block_timestamp, epoch_id, next_epoch_id, chunks_included, \
                        author_id, protocol_version, gas_price, block_ordinal, \
                        total_supply, num_transactions, num_receipts, gas_burnt, tokens_burnt \
                     FROM blocks WHERE block_hash = '{}' LIMIT 1",
                    hash
                )
            }
            BlockId::Height(height) => {
                format!(
                    "SELECT \
                        block_height, prev_block_height, block_hash, prev_block_hash, \
                        block_timestamp, epoch_id, next_epoch_id, chunks_included, \
                        author_id, protocol_version, gas_price, block_ordinal, \
                        total_supply, num_transactions, num_receipts, gas_burnt, tokens_burnt \
                     FROM blocks WHERE block_height = {} LIMIT 1",
                    height
                )
            }
        };
        let rows: Vec<BlockRow> = self.read_rows(&query).await?;
        Ok(rows.into_iter().next())
    }

    pub async fn get_block_txs(
        &self,
        block_height: BlockHeight,
    ) -> clickhouse::error::Result<Vec<BlockTxRow>> {
        let query = format!(
            "SELECT \
                transaction_hash, signer_id, tx_block_height, tx_index, tx_block_hash, \
                tx_block_timestamp, last_block_height, is_completed, shard_id, receiver_id, \
                signer_public_key, priority_fee, nonce, is_relayed, real_signer_id, \
                real_receiver_id, is_success, gas_burnt, tokens_burnt \
             FROM transactions \
             WHERE tx_block_height = {} \
             ORDER BY tx_index",
            block_height
        );
        let mut rows: Vec<BlockTxRow> = self.read_rows(&query).await?;
        rows.dedup_by_key(|r| r.tx_index);
        Ok(rows)
    }

    pub async fn get_block_receipts(
        &self,
        block_height: BlockHeight,
    ) -> clickhouse::error::Result<Vec<ReceiptTxRow>> {
        let query = format!(
            "SELECT \
                receipt_id, block_height, block_timestamp, receipt_index, \
                appear_block_height, appear_receipt_index, transaction_hash, \
                tx_block_height, tx_block_timestamp, tx_index, predecessor_id, \
                receiver_id, receipt_type, priority, shard_id, is_success \
             FROM receipt_txs \
             WHERE block_height = {} \
             ORDER BY receipt_index, receipt_id",
            block_height
        );
        let mut rows: Vec<ReceiptTxRow> = self.read_rows(&query).await?;
        rows.dedup_by_key(|r| (r.receipt_index, r.receipt_id.clone()));
        Ok(rows)
    }

    pub async fn get_blocks(
        &self,
        from_block_height: Option<BlockHeight>,
        to_block_height: Option<BlockHeight>,
        limit: usize,
        desc: bool,
    ) -> clickhouse::error::Result<Vec<BlockRow>> {
        let mut conditions = Vec::new();

        if let Some(from) = from_block_height {
            conditions.push(format!("block_height >= {}", from));
        }
        if let Some(to) = to_block_height {
            conditions.push(format!("block_height <= {}", to));
        }

        let where_clause = if conditions.is_empty() {
            String::new()
        } else {
            format!("WHERE {}", conditions.join(" AND "))
        };

        let order = if desc { "DESC" } else { "ASC" };
        let table = if conditions.is_empty() && desc {
            "blocks_latest"
        } else {
            "blocks"
        };
        let query = format!(
            "SELECT \
                block_height, prev_block_height, block_hash, prev_block_hash, \
                block_timestamp, epoch_id, next_epoch_id, chunks_included, \
                author_id, protocol_version, gas_price, block_ordinal, \
                total_supply, num_transactions, num_receipts, gas_burnt, tokens_burnt \
             FROM {table} \
             {} \
             ORDER BY block_height {} \
             LIMIT {}",
            where_clause, order, limit
        );
        self.read_rows(&query).await
    }

    pub async fn get_receipt(
        &self,
        receipt_id: &str,
    ) -> clickhouse::error::Result<Option<ReceiptTxRow>> {
        let query = format!(
            "SELECT \
                receipt_id, block_height, block_timestamp, receipt_index, \
                appear_block_height, appear_receipt_index, transaction_hash, \
                tx_block_height, tx_block_timestamp, tx_index, predecessor_id, \
                receiver_id, receipt_type, priority, shard_id, is_success \
             FROM receipt_txs \
             WHERE receipt_id = '{}' \
             LIMIT 1",
            receipt_id
        );
        let rows: Vec<ReceiptTxRow> = self.read_rows(&query).await?;
        Ok(rows.into_iter().next())
    }
}

fn establish_connection() -> Client {
    Client::default()
        .with_url(env::var("DATABASE_URL").unwrap())
        .with_user(env::var("DATABASE_USER").unwrap())
        .with_password(env::var("DATABASE_PASSWORD").unwrap())
        .with_database(env::var("DATABASE_DATABASE").unwrap())
}
