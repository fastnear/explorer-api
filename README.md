# FASTNEAR's Explorer API server

The server uses Clickhouse database to query and return data to the client.
See https://github.com/fastnear/clickhouse-provider/tree/explorer for the Clickhouse database setup.

The following API endpoints are available:

- Transactions
- Accounts
- Block and latest blocks
- Receipts

## Transactions

**POST** `/v0/transactions`

Returns transactions by hash.

#### Input

- `tx_hashes`: list of transaction hashes (limit 20 hashes)

#### Output

- `transactions`: list of matching transactions

#### Example

Get two transactions by hashes:

```bash
http post http://localhost:3000/v0/transactions tx_hashes:='["HV42hanyDVK3MYoW8c17Ufxw83htNEQGk93bzyQRocvS", "D2pt9ceGUyiGdJeeZLDd3uBGHF11DmPe4aJFMaoWMmDd"]'
```

<details>
  <summary><b>Show Output</b></summary>

```json
{
  "transactions": [{
    "data_receipts": [{
      "data_id": "D4yvigdBjzYU1g2N9hvnzkeV7MCyepnLnRETVtDu9YnV",
      "receipt_id": "DKguTtMeWCvxyrLPGU9x1rCk5x8msLa7tJM9Pp5AEL1A",
      "receiver_id": "staking-pool.sweatmint.near",
      "predecessor_id": "token.sweat",
      "receipt": {
        "Data": {
          "data": "",
          "data_id": "D4yvigdBjzYU1g2N9hvnzkeV7MCyepnLnRETVtDu9YnV"
        }
      }
    }],
    "execution_outcome": {
      "block_hash": "8dB8ZPYrnnHrYjmNSavK7Gx2UyHLdAkc5jeFmurku5jh",
      "id": "HV42hanyDVK3MYoW8c17Ufxw83htNEQGk93bzyQRocvS",
      "outcome": {
        "executor_id": "lenguyenwin.near",
        "gas_burnt": 2428019381096,
        "logs": [],
        "receipt_ids": ["H6Roj3A2SNn7HJXdigLPDHiz2kcob4MHwLg3nnDEH2VZ"],
        "status": {
          "SuccessReceiptId": "H6Roj3A2SNn7HJXdigLPDHiz2kcob4MHwLg3nnDEH2VZ"
        },
        "tokens_burnt": "242801938109600000000",
        "metadata": {
          "gas_profile": null,
          "version": 1
        }
      },
      "proof": []
    },
    "receipts": [{
      "predecessor_id": "lenguyenwin.near",
      "receipt": {
        "Action": {
          "actions": [{
            "FunctionCall": {
              "args": "eyJzdGFraW5nX3BhY2thZ2VfaWQiOjQ4Nn0=",
              "deposit": "1",
              "gas": 60000000000000,
              "method_name": "claim_stake_reward"
            }
          }],
          "gas_price": "146853372",
          "input_data_ids": [],
          "output_data_receivers": [],
          "signer_id": "lenguyenwin.near",
          "signer_public_key": "ed25519:4iW7dw89iZrYNbbewyZu2DhuRcpfiLSCqut9aT5x79B1"
        }
      },
      "receipt_id": "H6Roj3A2SNn7HJXdigLPDHiz2kcob4MHwLg3nnDEH2VZ",
      "receiver_id": "staking-pool.sweatmint.near"
    }],
    "transaction": {
      "actions": [{
        "FunctionCall": {
          "args": "eyJzdGFraW5nX3BhY2thZ2VfaWQiOjQ4Nn0=",
          "deposit": "1",
          "gas": 60000000000000,
          "method_name": "claim_stake_reward"
        }
      }],
      "hash": "HV42hanyDVK3MYoW8c17Ufxw83htNEQGk93bzyQRocvS",
      "nonce": 83145267000161,
      "public_key": "ed25519:4iW7dw89iZrYNbbewyZu2DhuRcpfiLSCqut9aT5x79B1",
      "receiver_id": "staking-pool.sweatmint.near",
      "signature": "ed25519:2SAM6JuqwaJzGoEQ1LxPAKQKG235tnfojFrpTiUQzqkD1rprCZdhBz5wHqYorbrcz1F1mnPFs5Kb2p7K2uxCD5Y3",
      "signer_id": "lenguyenwin.near"
    }
  }]
}
```

</details>

## Accounts

**POST** `/v0/account`

Returns the list of transaction metadata associated with the given account ID.
Associated transactions are transactions signed by the account or transactions that are relevant to the account (e.g.
transactions that transfer tokens to the account).

The method may take the `max_block_height` parameter to support pagination.
If the `max_block_height` is provided, the method will return transactions with block heights less than or equal to the
given `max_block_height`.

When `max_block_height` is not provided, the method will return the most recent transactions and the total number of
transactions associated with the account.

- Up to 200 transaction metadata rows will be returned.
- Up to 20 transactions will be expanded from the matched transaction metadata rows.

#### Input

- `account_id` - The account ID to query.
- *(optional)* `max_block_height` - The maximum block height for the transactions to return.

#### Output

- `account_txs` - The list of transaction metadata rows associated with the account.
  - `account_id` - The account ID.
  - `signer_id` - The signer ID.
  - `transaction_hash` - The transaction hash.
  - `tx_block_height` - The block height of the transaction.
  - `tx_block_timestamp` - The block timestamp of the transaction.
- `transactions` - The list of the first 20 matching associated transactions.
- `total_txs` - The total number of transactions associated with the account. Only returned if the `max_block_height` is
  not provided.

#### Example

Get recent transactions for the `sweat_welcome.near` account:

```bash
http post http://localhost:3000/v0/account account_id:='"sweat_welcome.near"'
```

<details>
  <summary><b>Show Output</b></summary>

```json5
{
  "account_txs": [
    {
      "account_id": "sweat_welcome.near",
      "signer_id": "sweat_welcome.near",
      "transaction_hash": "Er4gubtM5rUgMakSDzP9n7FVyUjZZkxsKJFCwoMywQgV",
      "tx_block_height": 96098994,
      "tx_block_timestamp": 1688940091339135477
    },
    // ... more transaction metadata rows
  ],
  "transactions": [
    {
      "transaction": {
        "actions": [{
          "FunctionCall": {
            "args": "eyJhY2NvdW50X2lkIjoiYWJjMTc2YmU2ZDYzY2FmN2FlMzNjYWYzNDYzYmU2ZjkxMGI4YmI5ZWZiNmJmZWMzZjI2YTY2NDBhMWI0NmYwNCJ9",
            "deposit": "1250000000000000000000",
            "gas": 30000000000000,
            "method_name": "storage_deposit"
          }
        }],
        "hash": "2bCYEdSzAojeQe8BYqbFRZJxaZS8ZMfCQt7gtDabcN3W",
        "nonce": 64885790401249,
        "public_key": "ed25519:D6cHxv3s9wYiWyhsqzKfqQm6XW4fhGS4Eg97U41v3zbh",
        "receiver_id": "token.sweat",
        "signer_id": "sweat_welcome.near"
      },
      "execution_outcome": {
        "block_hash": "ASDm9EzkkCfT89AtX8XUmv2fyFaAobDUZmnsiueGHsJ8",
        "outcome": {
          "executor_id": "sweat_welcome.near",
          "gas_burnt": 2428135649664,
          "status": {
            "SuccessReceiptId": "HpQmCSFMNd3ZuHCSw7wFLBzpwQEqR9XfY8xDTBey8Aak"
          }
          // ... additional outcome fields omitted
        }
      }
      // ... additional fields like receipts, data_receipts omitted
    },
    // ... more transactions
  ],
  "txs_count": 21854769
}
```

</details>

#### Example

Get transactions for the `near` account with a maximum block height of `10000000`:

```bash
http post http://localhost:3000/v0/account account_id:='"near"' max_block_height:=10000000
```

<details>
  <summary><b>Show Output</b></summary>

```json5
{
  "account_txs": [
    {
      "account_id": "near",
      "signer_id": "near",
      "transaction_hash": "2hprGWVmVhQ2uq2Yda7CPTuqG7vJLrUm1GNTM5A4xGoQ",
      "tx_block_height": 9823941,
      "tx_block_timestamp": 1595371606784154928
    }
    // ... more transaction metadata rows
  ],
  "transactions": [
    {
      "transaction": {
        "actions": [
          "CreateAccount",
          {
            "Transfer": {
              "deposit": "1000000000000000000000000"
            }
          },
          {
            "AddKey": {
              "access_key": {
                "nonce": 0,
                "permission": "FullAccess"
              },
              "public_key": "ed25519:HbSrzVndedNoLZFD6FHRwStRntLHWfAprzoSdEF81fLi"
            }
          }
        ],
        "hash": "FRBTkEzjLEQekWmKzKfHoTQQJH6Fi5EKxuACWvjBP9wN",
        "nonce": 15,
        "public_key": "ed25519:5zset1JX4qp4PcR3N9KDSY6ATdgkrbBW5wFBGWC4ZjnU",
        "receiver_id": "henry.near",
        "signer_id": "near"
      },
      "execution_outcome": {
        "block_hash": "aiWXsSZZzQtaj82rq6Lk6sF4nXqi1GMMN2yKHVozwNB",
        "outcome": {
          "executor_id": "near",
          "gas_burnt": 424555062500,
          "status": {
            "SuccessReceiptId": "CyTJV9kcp6eLrLZbX9hDZQLZ7BJKzMpx7kHMWnhL2v8Z"
          }
          // ... additional outcome fields omitted
        }
      }
      // ... additional fields like receipts, data_receipts omitted
    }
    // ... more transactions
  ]
}
```

</details>

## Block

**POST** `/v0/block`

Returns the list of transactions associated with a block at the given block ID.

The associated transactions are transactions that originated in the block or had a receipt in the block.

#### Input

- `block_id`: Either the block height (if an integer is given) or the block hash (if a string is given) of the block to
  query.

#### Output

- `block_txs` - The list of transactions associated with the block.
- `transactions` - The list of the first 20 matching associated transactions.

#### Example

Get the transactions associated with block 9823941.

```bash
http post http://localhost:3000/v0/block block_id:=9823941
```

<details>
  <summary><b>Show Output</b></summary>

```json
{
  "block_txs": [
    {
      "block_hash": "EBGVpv1MGSo3eTKMdzueRcQG5urfG2MDqUad6ErCX1EF",
      "block_height": 9823941,
      "block_timestamp": 1595371606784154928,
      "signer_id": "near",
      "transaction_hash": "2hprGWVmVhQ2uq2Yda7CPTuqG7vJLrUm1GNTM5A4xGoQ",
      "tx_block_height": 9823941
    }
  ],
  "transactions": [
    {
      "data_receipts": [],
      "execution_outcome": {
        "block_hash": "EBGVpv1MGSo3eTKMdzueRcQG5urfG2MDqUad6ErCX1EF",
        "id": "2hprGWVmVhQ2uq2Yda7CPTuqG7vJLrUm1GNTM5A4xGoQ",
        "outcome": {
          "executor_id": "near",
          "gas_burnt": 424555062500,
          "logs": [],
          "metadata": {
            "gas_profile": null,
            "version": 1
          },
          "receipt_ids": [
            "4GnCfvGFKUasMUHGCBokrj9LLGRmZWFMas1xZTihtVTK"
          ],
          "status": {
            "SuccessReceiptId": "4GnCfvGFKUasMUHGCBokrj9LLGRmZWFMas1xZTihtVTK"
          },
          "tokens_burnt": "424555062500000000000"
        },
        "proof": []
      },
      "receipts": [
        {
          "execution_outcome": {
            "block_hash": "EHQNSBxA43BSXGLZSetrH9hZLxbVuEPHRdRcEcBmTV7A",
            "id": "4GnCfvGFKUasMUHGCBokrj9LLGRmZWFMas1xZTihtVTK",
            "outcome": {
              "executor_id": "fresh.near",
              "gas_burnt": 424555062500,
              "logs": [],
              "metadata": {
                "gas_profile": null,
                "version": 1
              },
              "receipt_ids": [
                "GopsQ9mq8NDKZYuXFzxJuCeop35fEa4jt8RJMJuhW5ZB"
              ],
              "status": {
                "SuccessValue": ""
              },
              "tokens_burnt": "424555062500000000000"
            },
            "proof": []
          },
          "receipt": {
            "predecessor_id": "near",
            "receipt": {
              "Action": {
                "actions": [
                  "CreateAccount",
                  {
                    "Transfer": {
                      "deposit": "1000000000000000000000000"
                    }
                  },
                  {
                    "AddKey": {
                      "access_key": {
                        "nonce": 0,
                        "permission": "FullAccess"
                      },
                      "public_key": "ed25519:39mtn6H92UR82avZx8bvyNZ2UuCHL2JnCxMecM9dUMQa"
                    }
                  }
                ],
                "gas_price": "1030000000",
                "input_data_ids": [],
                "output_data_receivers": [],
                "signer_id": "near",
                "signer_public_key": "ed25519:5zset1JX4qp4PcR3N9KDSY6ATdgkrbBW5wFBGWC4ZjnU"
              }
            },
            "receipt_id": "4GnCfvGFKUasMUHGCBokrj9LLGRmZWFMas1xZTihtVTK",
            "receiver_id": "fresh.near"
          }
        },
        {
          "execution_outcome": {
            "block_hash": "DwQ3D1PhftrkZ4h9B2RJkaCTy1VJU5aKRLbKdyr9LUft",
            "id": "GopsQ9mq8NDKZYuXFzxJuCeop35fEa4jt8RJMJuhW5ZB",
            "outcome": {
              "executor_id": "near",
              "gas_burnt": 0,
              "logs": [],
              "metadata": {
                "gas_profile": null,
                "version": 1
              },
              "receipt_ids": [],
              "status": {
                "SuccessValue": ""
              },
              "tokens_burnt": "0"
            },
            "proof": []
          },
          "receipt": {
            "predecessor_id": "system",
            "receipt": {
              "Action": {
                "actions": [
                  {
                    "Transfer": {
                      "deposit": "12736651875000000000"
                    }
                  }
                ],
                "gas_price": "0",
                "input_data_ids": [],
                "output_data_receivers": [],
                "signer_id": "near",
                "signer_public_key": "ed25519:5zset1JX4qp4PcR3N9KDSY6ATdgkrbBW5wFBGWC4ZjnU"
              }
            },
            "receipt_id": "GopsQ9mq8NDKZYuXFzxJuCeop35fEa4jt8RJMJuhW5ZB",
            "receiver_id": "near"
          }
        }
      ],
      "transaction": {
        "actions": [
          "CreateAccount",
          {
            "Transfer": {
              "deposit": "1000000000000000000000000"
            }
          },
          {
            "AddKey": {
              "access_key": {
                "nonce": 0,
                "permission": "FullAccess"
              },
              "public_key": "ed25519:39mtn6H92UR82avZx8bvyNZ2UuCHL2JnCxMecM9dUMQa"
            }
          }
        ],
        "hash": "2hprGWVmVhQ2uq2Yda7CPTuqG7vJLrUm1GNTM5A4xGoQ",
        "nonce": 17,
        "public_key": "ed25519:5zset1JX4qp4PcR3N9KDSY6ATdgkrbBW5wFBGWC4ZjnU",
        "receiver_id": "fresh.near",
        "signature": "ed25519:fvS1kYu9KseAH19eYDrrzPSonTCSvbUPa5f2gNbNBfnX82B5d4Xz4J7abZ7eBULPWRdwRaK6BR9uxXy7g3kXNSF",
        "signer_id": "near"
      }
    }
  ]
}
```

</details>

#### Example

Get the transactions associated with block hash `EBGVpv1MGSo3eTKMdzueRcQG5urfG2MDqUad6ErCX1EF`.

```bash
http post http://localhost:3000/v0/block block_id:='"EBGVpv1MGSo3eTKMdzueRcQG5urfG2MDqUad6ErCX1EF"'
```

<details>
  <summary><b>Show Output</b></summary>

```json
{
  "block_txs": [
    {
      "block_hash": "EBGVpv1MGSo3eTKMdzueRcQG5urfG2MDqUad6ErCX1EF",
      "block_height": 9823941,
      "block_timestamp": 1595371606784154928,
      "signer_id": "near",
      "transaction_hash": "2hprGWVmVhQ2uq2Yda7CPTuqG7vJLrUm1GNTM5A4xGoQ",
      "tx_block_height": 9823941
    }
  ],
  "transactions": [
    {
      "data_receipts": [],
      "execution_outcome": {
        "block_hash": "EBGVpv1MGSo3eTKMdzueRcQG5urfG2MDqUad6ErCX1EF",
        "id": "2hprGWVmVhQ2uq2Yda7CPTuqG7vJLrUm1GNTM5A4xGoQ",
        "outcome": {
          "executor_id": "near",
          "gas_burnt": 424555062500,
          "logs": [],
          "metadata": {
            "gas_profile": null,
            "version": 1
          },
          "receipt_ids": [
            "4GnCfvGFKUasMUHGCBokrj9LLGRmZWFMas1xZTihtVTK"
          ],
          "status": {
            "SuccessReceiptId": "4GnCfvGFKUasMUHGCBokrj9LLGRmZWFMas1xZTihtVTK"
          },
          "tokens_burnt": "424555062500000000000"
        },
        "proof": []
      },
      "receipts": [
        {
          "execution_outcome": {
            "block_hash": "EHQNSBxA43BSXGLZSetrH9hZLxbVuEPHRdRcEcBmTV7A",
            "id": "4GnCfvGFKUasMUHGCBokrj9LLGRmZWFMas1xZTihtVTK",
            "outcome": {
              "executor_id": "fresh.near",
              "gas_burnt": 424555062500,
              "logs": [],
              "metadata": {
                "gas_profile": null,
                "version": 1
              },
              "receipt_ids": [
                "GopsQ9mq8NDKZYuXFzxJuCeop35fEa4jt8RJMJuhW5ZB"
              ],
              "status": {
                "SuccessValue": ""
              },
              "tokens_burnt": "424555062500000000000"
            },
            "proof": []
          },
          "receipt": {
            "predecessor_id": "near",
            "receipt": {
              "Action": {
                "actions": [
                  "CreateAccount",
                  {
                    "Transfer": {
                      "deposit": "1000000000000000000000000"
                    }
                  },
                  {
                    "AddKey": {
                      "access_key": {
                        "nonce": 0,
                        "permission": "FullAccess"
                      },
                      "public_key": "ed25519:39mtn6H92UR82avZx8bvyNZ2UuCHL2JnCxMecM9dUMQa"
                    }
                  }
                ],
                "gas_price": "1030000000",
                "input_data_ids": [],
                "output_data_receivers": [],
                "signer_id": "near",
                "signer_public_key": "ed25519:5zset1JX4qp4PcR3N9KDSY6ATdgkrbBW5wFBGWC4ZjnU"
              }
            },
            "receipt_id": "4GnCfvGFKUasMUHGCBokrj9LLGRmZWFMas1xZTihtVTK",
            "receiver_id": "fresh.near"
          }
        },
        {
          "execution_outcome": {
            "block_hash": "DwQ3D1PhftrkZ4h9B2RJkaCTy1VJU5aKRLbKdyr9LUft",
            "id": "GopsQ9mq8NDKZYuXFzxJuCeop35fEa4jt8RJMJuhW5ZB",
            "outcome": {
              "executor_id": "near",
              "gas_burnt": 0,
              "logs": [],
              "metadata": {
                "gas_profile": null,
                "version": 1
              },
              "receipt_ids": [],
              "status": {
                "SuccessValue": ""
              },
              "tokens_burnt": "0"
            },
            "proof": []
          },
          "receipt": {
            "predecessor_id": "system",
            "receipt": {
              "Action": {
                "actions": [
                  {
                    "Transfer": {
                      "deposit": "12736651875000000000"
                    }
                  }
                ],
                "gas_price": "0",
                "input_data_ids": [],
                "output_data_receivers": [],
                "signer_id": "near",
                "signer_public_key": "ed25519:5zset1JX4qp4PcR3N9KDSY6ATdgkrbBW5wFBGWC4ZjnU"
              }
            },
            "receipt_id": "GopsQ9mq8NDKZYuXFzxJuCeop35fEa4jt8RJMJuhW5ZB",
            "receiver_id": "near"
          }
        }
      ],
      "transaction": {
        "actions": [
          "CreateAccount",
          {
            "Transfer": {
              "deposit": "1000000000000000000000000"
            }
          },
          {
            "AddKey": {
              "access_key": {
                "nonce": 0,
                "permission": "FullAccess"
              },
              "public_key": "ed25519:39mtn6H92UR82avZx8bvyNZ2UuCHL2JnCxMecM9dUMQa"
            }
          }
        ],
        "hash": "2hprGWVmVhQ2uq2Yda7CPTuqG7vJLrUm1GNTM5A4xGoQ",
        "nonce": 17,
        "public_key": "ed25519:5zset1JX4qp4PcR3N9KDSY6ATdgkrbBW5wFBGWC4ZjnU",
        "receiver_id": "fresh.near",
        "signature": "ed25519:fvS1kYu9KseAH19eYDrrzPSonTCSvbUPa5f2gNbNBfnX82B5d4Xz4J7abZ7eBULPWRdwRaK6BR9uxXy7g3kXNSF",
        "signer_id": "near"
      }
    }
  ]
}
```

</details>

## Last Blocks

**POST** `/v0/blocks/last`

Returns the number of associated transactions for each of the last 10 block.

The associated transactions are transactions that originated in the block or had a receipt in the block.

#### Input

*None*

#### Output

- `blocks` - The list of the last 10 blocks and transaction counts. Includes:
  - `block_hash` - The block hash.
  - `block_height` - The block height.
  - `block_timestamp` - The block timestamp in nanoseconds.
  - `txs_count` - The number of associated transactions for the block

#### Example

```bash
http post http://localhost:3000/v0/blocks/last
```

<details>
  <summary><b>Show Output</b></summary>

```json
{
  "blocks": [
    {
      "block_hash": "57DtQ9JSuiF4YnNuLkKxUdVE5DWGKfeNfJqzpqzunn6u",
      "block_height": 120153000,
      "block_timestamp": 1717182115682386560,
      "txs_count": 104
    },
    {
      "block_hash": "CG9FAixL3xohtDJtibxSxyNwkpyW4ndcHpRjRFrZXzPm",
      "block_height": 120152999,
      "block_timestamp": 1717182114391384635,
      "txs_count": 186
    },
    {
      "block_hash": "4Mcu5i3txEer5oWkC1WoqLMJdpMFAXMMhCn8fUevc3Q5",
      "block_height": 120152998,
      "block_timestamp": 1717182113137503632,
      "txs_count": 315
    },
    {
      "block_hash": "GsW2HGLaWvhkg6hAFJxnL8mCUwp3MveoaxdNAhyJ5TJk",
      "block_height": 120152997,
      "block_timestamp": 1717182111869198228,
      "txs_count": 378
    },
    {
      "block_hash": "3yTBuwvZHnEDCrbCJF77ScVuxtrY7VBjWNP1b4KHsRwb",
      "block_height": 120152996,
      "block_timestamp": 1717182110830816530,
      "txs_count": 355
    },
    {
      "block_hash": "8zfCnpaeSeXJ7MNzg8xvGEQiPpNoDC8fWqHWZp5guuSn",
      "block_height": 120152995,
      "block_timestamp": 1717182109815921046,
      "txs_count": 360
    },
    {
      "block_hash": "9GvjxYB5R9CLrogiZatXtCZ4jFVjtzrYqxskaNYG8qTn",
      "block_height": 120152994,
      "block_timestamp": 1717182108789884315,
      "txs_count": 330
    },
    {
      "block_hash": "4V5PgLLAr5TbRqMZgVhy5h8e8sPWn8AgdHJG45dcGhPK",
      "block_height": 120152993,
      "block_timestamp": 1717182107691236770,
      "txs_count": 366
    },
    {
      "block_hash": "32tUbpRLKZ4C8wMD88EKqSbJDA2oDXyAK1vrBt3JvVzb",
      "block_height": 120152992,
      "block_timestamp": 1717182106546208694,
      "txs_count": 381
    },
    {
      "block_hash": "6yFba9MHoZBbfzTRQaE394JjdeNLdYr944fDQhhLnwGY",
      "block_height": 120152991,
      "block_timestamp": 1717182105425344712,
      "txs_count": 393
    }
  ]
}
```

</details>

## Receipt

**POST** `/v0/receipt`

Returns the transaction that includes the given receipt ID.

#### Input

- `receipt_id`: The receipt ID to query.

#### Output

- `transaction` - The transaction that includes the given receipt ID (or `null`).

#### Example

Get the transaction that includes the receipt ID `Fgtwfcbt6p1bVTBEeycKaHaAbnYRfn6rTTxNo9RfvsQx`.

```bash
http post http://localhost:3000/v0/receipt receipt_id:='"Fgtwfcbt6p1bVTBEeycKaHaAbnYRfn6rTTxNo9RfvsQx"'
```

<details>
  <summary><b>Show Output</b></summary>

```json
{
  "transaction": {
    "data_receipts": [],
    "execution_outcome": {
      "block_hash": "ABuEm9AMsLN3yHB7WUUapigcwJTaDn6ZDVWGHjRhP1Ma",
      "id": "TUVnGuUaBt15eUoL3gWa4HJc9wDeutHKSHELDBxFeCT",
      "outcome": {
        "executor_id": "near",
        "gas_burnt": 424555062500,
        "logs": [],
        "metadata": {
          "gas_profile": null,
          "version": 1
        },
        "receipt_ids": [
          "4Apj4q6Nom3w96ek8Zy3TLcCojXHuVBt6E7Qcw6XZdQ2"
        ],
        "status": {
          "SuccessReceiptId": "4Apj4q6Nom3w96ek8Zy3TLcCojXHuVBt6E7Qcw6XZdQ2"
        },
        "tokens_burnt": "424555062500000000000"
      },
      "proof": []
    },
    "receipts": [
      {
        "execution_outcome": {
          "block_hash": "GXdEhC56a1DiqWic88d5EwtRGmHjb8ZTCjtS5ARhPkHt",
          "id": "4Apj4q6Nom3w96ek8Zy3TLcCojXHuVBt6E7Qcw6XZdQ2",
          "outcome": {
            "executor_id": "dokia.near",
            "gas_burnt": 424555062500,
            "logs": [],
            "metadata": {
              "gas_profile": null,
              "version": 1
            },
            "receipt_ids": [
              "Fgtwfcbt6p1bVTBEeycKaHaAbnYRfn6rTTxNo9RfvsQx"
            ],
            "status": {
              "SuccessValue": ""
            },
            "tokens_burnt": "424555062500000000000"
          },
          "proof": []
        },
        "receipt": {
          "predecessor_id": "near",
          "receipt": {
            "Action": {
              "actions": [
                "CreateAccount",
                {
                  "Transfer": {
                    "deposit": "50000000000000000000000000"
                  }
                },
                {
                  "AddKey": {
                    "access_key": {
                      "nonce": 0,
                      "permission": "FullAccess"
                    },
                    "public_key": "ed25519:8ZpjTDAxPkhmsgsACuvhorjuQsTVMvdB6Hbyf2Ciw9Ut"
                  }
                }
              ],
              "gas_price": "1030000000",
              "input_data_ids": [],
              "output_data_receivers": [],
              "signer_id": "near",
              "signer_public_key": "ed25519:5zset1JX4qp4PcR3N9KDSY6ATdgkrbBW5wFBGWC4ZjnU"
            }
          },
          "receipt_id": "4Apj4q6Nom3w96ek8Zy3TLcCojXHuVBt6E7Qcw6XZdQ2",
          "receiver_id": "dokia.near"
        }
      },
      {
        "execution_outcome": {
          "block_hash": "EUbMGS8WpfTwWKNTYm7WFhytWLiw7L4LNDjcn5QxGRmA",
          "id": "Fgtwfcbt6p1bVTBEeycKaHaAbnYRfn6rTTxNo9RfvsQx",
          "outcome": {
            "executor_id": "near",
            "gas_burnt": 0,
            "logs": [],
            "metadata": {
              "gas_profile": null,
              "version": 1
            },
            "receipt_ids": [],
            "status": {
              "SuccessValue": ""
            },
            "tokens_burnt": "0"
          },
          "proof": []
        },
        "receipt": {
          "predecessor_id": "system",
          "receipt": {
            "Action": {
              "actions": [
                {
                  "Transfer": {
                    "deposit": "12736651875000000000"
                  }
                }
              ],
              "gas_price": "0",
              "input_data_ids": [],
              "output_data_receivers": [],
              "signer_id": "near",
              "signer_public_key": "ed25519:5zset1JX4qp4PcR3N9KDSY6ATdgkrbBW5wFBGWC4ZjnU"
            }
          },
          "receipt_id": "Fgtwfcbt6p1bVTBEeycKaHaAbnYRfn6rTTxNo9RfvsQx",
          "receiver_id": "near"
        }
      }
    ],
    "transaction": {
      "actions": [
        "CreateAccount",
        {
          "Transfer": {
            "deposit": "50000000000000000000000000"
          }
        },
        {
          "AddKey": {
            "access_key": {
              "nonce": 0,
              "permission": "FullAccess"
            },
            "public_key": "ed25519:8ZpjTDAxPkhmsgsACuvhorjuQsTVMvdB6Hbyf2Ciw9Ut"
          }
        }
      ],
      "hash": "TUVnGuUaBt15eUoL3gWa4HJc9wDeutHKSHELDBxFeCT",
      "nonce": 13,
      "public_key": "ed25519:5zset1JX4qp4PcR3N9KDSY6ATdgkrbBW5wFBGWC4ZjnU",
      "receiver_id": "dokia.near",
      "signature": "ed25519:4gbN8XfoEvUweKeWvieB94XR1yQo6A2LZt1gjY6ByvPKNMh7wNY5qve87MJeRuMaGo8B49Dv7Dd3XfwFUeMm69AG",
      "signer_id": "near"
    }
  }
}
```

</details>


---
