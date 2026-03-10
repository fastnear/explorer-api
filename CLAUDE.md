# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build & Run

```bash
cargo build                # Debug build
cargo build --release      # Release build
cargo run                  # Run server (requires .env with ClickHouse credentials)
cargo test                 # Run tests
```

The server reads configuration from `.env` (DATABASE_URL, DATABASE_USER, DATABASE_PASSWORD, DATABASE_DATABASE, PORT). It binds to `127.0.0.1:PORT` (default 3000).

## Architecture

This is a Rust REST API server for querying NEAR blockchain data from a ClickHouse backend. It uses Actix-web and serves all endpoints as POST under `/v0`.

**Source layout:**
- `src/main.rs` — Server bootstrap, CORS config, route registration. Holds `AppState` (shared `ClickDB` instance).
- `src/api.rs` — All endpoint handlers in `mod v0`. Each handler deserializes JSON input, builds query parameters, calls `ClickDB` methods, and returns JSON. Error types: `ClickhouseError` (500) and `ArgumentError` (400).
- `src/click.rs` — `ClickDB` struct wrapping the `clickhouse::Client`. All SQL query construction and execution lives here. Queries are built via string formatting. Results are deduplicated with `dedup_by_key` to handle ClickHouse replica inconsistencies.
- `src/types.rs` — Row structs (`AccountTxRow`, `BlockTxRow`, `ReceiptTxRow`, `BlockRow`, `BlockId`) with `#[derive(Row)]` for ClickHouse deserialization. Contains the ClickHouse CREATE TABLE DDL as comments above each struct — these are the canonical schema reference.
- `src/cache.rs` — Placeholder module (empty, TODO).
- `static/index.html` — API docs page served at `/`.

**Data flow:** HTTP request → `api.rs` handler → `click.rs` query builder → ClickHouse → dedup → JSON response.

**Key patterns:**
- `u128` fields (gas_price, total_supply, tokens_burnt) are serialized as strings via `serde_with::DisplayFromStr`
- Raw transaction data is stored zstd-compressed in `raw_tx` table and decompressed on read
- Cursor pagination uses a packed `u128` token encoding `(block_height << 32 | tx_index)`
- The `blocks_latest` table is used instead of `blocks` for unfiltered descending queries (latest blocks)

## ClickHouse Tables

The API expects tables from [fastnear/clickhouse-provider](https://github.com/fastnear/clickhouse-provider/tree/click-dist): `account_txs`, `transactions`, `receipt_txs`, `blocks`, `blocks_latest`, `raw_tx`. Schema DDL is documented in `src/types.rs` comments.
