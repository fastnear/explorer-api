# FastNear Transactions API server

Note, this server expects the database tables from the Clickhouse
indexer: https://github.com/fastnear/clickhouse-provider/tree/click-dist

All endpoints are POST and accept JSON body. The base path is `/v0`.

## POST `/v0/transactions`

Fetch raw transaction data by transaction hashes. Up to 20 hashes per request.

```bash
curl -X POST http://localhost:3000/v0/transactions \
  -H 'Content-Type: application/json' \
  -d '{
    "tx_hashes": [
      "HV42hanyDVK3MYoW8c17Ufxw83htNEQGk93bzyQRocvS",
      "D2pt9ceGUyiGdJeeZLDd3uBGHF11DmPe4aJFMaoWMmDd"
    ]
  }'
```

Response:

```json5
{
  "transactions": [
    {
      /* raw transaction data */
    },
    ...
  ]
}
```

## POST `/v0/account`

Fetch account transaction history with filtering and cursor-based pagination.

Returns `account_txs` (list), `txs_count` (on first page), and `resume_token` (if more pages available).

**Required fields:**

- `account_id` - the account ID

**Optional filters:**

- `is_signer`, `is_delegated_signer`, `is_real_signer`, `is_any_signer` - signer role filters
- `is_predecessor`, `is_explicit_refund_to` - predecessor/refund filters
- `is_receiver`, `is_real_receiver` - receiver role filters
- `is_function_call`, `is_action_arg`, `is_event_log` - action type filters
- `is_success` - filter by transaction success status

**Pagination:**

- `limit` - number of results (default/max: 200)
- `desc` - sort order (default: true)
- `from_tx_block_height` / `to_tx_block_height` - block height range
- `resume_token` - cursor token from previous response

```bash
# Basic query
curl -X POST http://localhost:3000/v0/account \
  -H 'Content-Type: application/json' \
  -d '{
    "account_id": "intents.near"
  }'
  
# Pagination with resume_token
curl -X POST http://localhost:3000/v0/account \
  -H 'Content-Type: application/json' \
  -d '{
    "account_id": "intents.near",
    "resume_token": "796557291984781314"
  }'

# With filters
curl -X POST http://localhost:3000/v0/account \
  -H 'Content-Type: application/json' \
  -d '{
    "account_id": "intents.near",
    "is_real_signer": true,
    "is_success": true,
    "limit": 50
  }'

# Block height range
curl -X POST http://localhost:3000/v0/account \
  -H 'Content-Type: application/json' \
  -d '{
    "account_id": "intents.near",
    "from_tx_block_height": 150000000,
    "to_tx_block_height": 151000000,
    "desc": false
  }'
```

Response (first page):

```json5
{
  "txs_count": 81278,
  "account_txs": [
    {
      "account_id": "...",
      "transaction_hash": "...",
      "tx_block_height": ...,
      "tx_block_timestamp": "...",
      "tx_index": ...,
      "is_signer": ...,
      "is_delegated_signer": ...,
      "is_real_signer": ...,
      "is_any_signer": ...,
      "is_predecessor": ...,
      "is_explicit_refund_to": ...,
      "is_receiver": ...,
      "is_real_receiver": ...,
      "is_function_call": ...,
      "is_action_arg": ...,
      "is_event_log": ...,
      "is_success": ...
    },
    ...
  ],
  "resume_token": "..."
  // present if more pages available
}
```

## POST `/v0/block`

Fetch a block by height or hash. Optionally include transactions and/or receipts.

**Required fields:**

- `block_id` - block height (number) or block hash (string)

**Optional fields:**

- `with_transactions` - include block transactions (default: false)
- `with_receipts` - include block receipts (default: false)

```bash
# By block height
curl -X POST http://localhost:3000/v0/block \
  -H 'Content-Type: application/json' \
  -d '{
    "block_id": 130000000
  }'

# By block hash
curl -X POST http://localhost:3000/v0/block \
  -H 'Content-Type: application/json' \
  -d '{
    "block_id": "EeQ63DqKGRz1JPGQG7W9BpiGKmrF8ESn9nHr3djL8s1e"
  }'

# With transactions and receipts
curl -X POST http://localhost:3000/v0/block \
  -H 'Content-Type: application/json' \
  -d '{
    "block_id": 130000000,
    "with_transactions": true,
    "with_receipts": true
  }'
```

Response (with transactions and receipts):

```json5
{
  "block": {
    "block_height": ...,
    "prev_block_height": ...,
    "block_hash": "...",
    "prev_block_hash": "...",
    "block_timestamp": "...",
    "epoch_id": "...",
    "next_epoch_id": "...",
    "chunks_included": ...,
    "author_id": "...",
    "protocol_version": ...,
    "gas_price": "...",
    "block_ordinal": ...,
    "total_supply": "...",
    "num_transactions": ...,
    "num_receipts": ...,
    "gas_burnt": "...",
    "tokens_burnt": "..."
  },
  "block_txs": [
    // only if with_transactions: true
    {
      "transaction_hash": "...",
      "signer_id": "...",
      "tx_block_height": ...,
      "tx_index": ...,
      "tx_block_hash": "...",
      "tx_block_timestamp": "...",
      "last_block_height": ...,
      "is_completed": ...,
      "shard_id": ...,
      "receiver_id": "...",
      "signer_public_key": "...",
      "priority_fee": ...,
      "nonce": ...,
      "is_relayed": ...,
      "real_signer_id": "...",
      "real_receiver_id": "...",
      "is_success": ...,
      "gas_burnt": ...,
      "tokens_burnt": "..."
    },
    ...
  ],
  "block_receipts": [
    // only if with_receipts: true
    {
      "receipt_id": "...",
      "block_height": ...,
      "block_timestamp": "...",
      "receipt_index": ...,
      "appear_block_height": ...,
      "appear_receipt_index": ...,
      "transaction_hash": "...",
      "tx_block_height": ...,
      "tx_block_timestamp": "...",
      "tx_index": ...,
      "predecessor_id": "...",
      "receiver_id": "...",
      "receipt_type": "...",
      "priority": ...,
      "shard_id": ...,
      "is_success": ...
    },
    ...
  ]
}
```

## POST `/v0/blocks`

Fetch a list of blocks with optional height range and pagination.

**Optional fields:**

- `from_block_height` / `to_block_height` - block height range
- `limit` - number of results (default/max: 100)
- `desc` - sort order (default: true)

```bash
# Latest blocks
curl -X POST http://localhost:3000/v0/blocks \
  -H 'Content-Type: application/json' \
  -d '{}'

# With range and limit
curl -X POST http://localhost:3000/v0/blocks \
  -H 'Content-Type: application/json' \
  -d '{
    "from_block_height": 130000000,
    "to_block_height": 130000100,
    "limit": 10,
    "desc": false
  }'
```

Response:

```json5
{
  "blocks": [
    {
      "block_height": ...,
      "prev_block_height": ...,
      "block_hash": "...",
      "prev_block_hash": "...",
      "block_timestamp": "...",
      "epoch_id": "...",
      "next_epoch_id": "...",
      "chunks_included": ...,
      "author_id": "...",
      "protocol_version": ...,
      "gas_price": "...",
      "block_ordinal": ...,
      "total_supply": "...",
      "num_transactions": ...,
      "num_receipts": ...,
      "gas_burnt": "...",
      "tokens_burnt": "..."
    },
    ...
  ]
}
```

## POST `/v0/receipt`

Fetch a receipt by ID. Returns the receipt details and the associated raw transaction.

**Required fields:**

- `receipt_id` - the receipt hash

```bash
curl -X POST http://localhost:3000/v0/receipt \
  -H 'Content-Type: application/json' \
  -d '{
    "receipt_id": "H6Roj3A2SNn7HJXdigLPDHiz2kcob4MHwLg3nnDEH2VZ"
  }'
```

Response:

```json5
{
  "receipt": {
    "receipt_id": "...",
    "block_height": ...,
    "block_timestamp": "...",
    "receipt_index": ...,
    "appear_block_height": ...,
    "appear_receipt_index": ...,
    "transaction_hash": "...",
    "tx_block_height": ...,
    "tx_block_timestamp": "...",
    "tx_index": ...,
    "predecessor_id": "...",
    "receiver_id": "...",
    "receipt_type": "...",
    "priority": ...,
    "shard_id": ...,
    "is_success": ...
  },
  "transaction": {
    /* raw transaction data */
  }
}
```
