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
  "transactions": [
    {
      "data_receipts": [],
      "execution_outcome": {
        "block_hash": "6kCXbQPKQGeaakpLjeVMNPRYZpgASgStzmp3APtHGigU",
        "id": "D2pt9ceGUyiGdJeeZLDd3uBGHF11DmPe4aJFMaoWMmDd",
        "outcome": {
          "executor_id": "sweat_welcome.near",
          "gas_burnt": 2428135649664,
          "logs": [],
          "metadata": {
            "gas_profile": null,
            "version": 1
          },
          "receipt_ids": [
            "D7xNE8SRhfPnCktXsfcLtKLPkyUz1zVuGCdHeWAojwyp"
          ],
          "status": {
            "SuccessReceiptId": "D7xNE8SRhfPnCktXsfcLtKLPkyUz1zVuGCdHeWAojwyp"
          },
          "tokens_burnt": "242813564966400000000"
        },
        "proof": []
      },
      "receipts": [
        {
          "execution_outcome": {
            "block_hash": "DtnJZKVRLj7UaYTsJ1JU8XqeC4kcGCTFUm8st8HT4iXz",
            "id": "D7xNE8SRhfPnCktXsfcLtKLPkyUz1zVuGCdHeWAojwyp",
            "outcome": {
              "executor_id": "token.sweat",
              "gas_burnt": 3283399376040,
              "logs": [],
              "metadata": {
                "gas_profile": null,
                "version": 3
              },
              "receipt_ids": [
                "F53ndtyjkg3tvvfC9WBFsqhutrfF4J3rW1Dp3yLnPVfj",
                "CzGKh7rF1zoBtqeHhLrK3uy8fhuYBPdPHiEfMqe1hr6V"
              ],
              "status": {
                "SuccessValue": "eyJ0b3RhbCI6IjkwMDAwMDAwMDAwMDAwMDAwMDAwMCIsImF2YWlsYWJsZSI6IjAifQ=="
              },
              "tokens_burnt": "328339937604000000000"
            },
            "proof": []
          },
          "receipt": {
            "predecessor_id": "sweat_welcome.near",
            "receipt": {
              "Action": {
                "actions": [
                  {
                    "FunctionCall": {
                      "args": "eyJhY2NvdW50X2lkIjoiZjhlZTc1NGM5YjMxMWJlM2Q4MzhjZjMwNGE5Mzc2ZDZjYTkzMzJiMTg4MWIwNTY5ODRmNzhiMjRhOWQzMjZjYiJ9",
                      "deposit": "1250000000000000000000",
                      "gas": 30000000000000,
                      "method_name": "storage_deposit"
                    }
                  }
                ],
                "gas_price": "122987387",
                "input_data_ids": [],
                "output_data_receivers": [],
                "signer_id": "sweat_welcome.near",
                "signer_public_key": "ed25519:2zZW3xeqAiKCiH1KbcrXWcNdkQXUrMUYHPYkZWiWfxtV"
              }
            },
            "receipt_id": "D7xNE8SRhfPnCktXsfcLtKLPkyUz1zVuGCdHeWAojwyp",
            "receiver_id": "token.sweat"
          }
        },
        {
          "execution_outcome": {
            "block_hash": "4mab4E2u6o5i4fnm4CEhtbKUwYq7h5nRb9yiaMhSzVmL",
            "id": "F53ndtyjkg3tvvfC9WBFsqhutrfF4J3rW1Dp3yLnPVfj",
            "outcome": {
              "executor_id": "sweat_welcome.near",
              "gas_burnt": 223182562500,
              "logs": [],
              "metadata": {
                "gas_profile": null,
                "version": 3
              },
              "receipt_ids": [
                "ueoqZz7ohFbbGCT9hkF6ZJbZyqw3MPdwLi3keCLhqkN"
              ],
              "status": {
                "SuccessValue": ""
              },
              "tokens_burnt": "22318256250000000000"
            },
            "proof": []
          },
          "receipt": {
            "predecessor_id": "token.sweat",
            "receipt": {
              "Action": {
                "actions": [
                  {
                    "Transfer": {
                      "deposit": "350000000000000000000"
                    }
                  }
                ],
                "gas_price": "122987387",
                "input_data_ids": [],
                "output_data_receivers": [],
                "signer_id": "sweat_welcome.near",
                "signer_public_key": "ed25519:2zZW3xeqAiKCiH1KbcrXWcNdkQXUrMUYHPYkZWiWfxtV"
              }
            },
            "receipt_id": "F53ndtyjkg3tvvfC9WBFsqhutrfF4J3rW1Dp3yLnPVfj",
            "receiver_id": "sweat_welcome.near"
          }
        },
        {
          "execution_outcome": {
            "block_hash": "4mab4E2u6o5i4fnm4CEhtbKUwYq7h5nRb9yiaMhSzVmL",
            "id": "CzGKh7rF1zoBtqeHhLrK3uy8fhuYBPdPHiEfMqe1hr6V",
            "outcome": {
              "executor_id": "sweat_welcome.near",
              "gas_burnt": 223182562500,
              "logs": [],
              "metadata": {
                "gas_profile": null,
                "version": 3
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
                      "deposit": "3632463091043883600468"
                    }
                  }
                ],
                "gas_price": "0",
                "input_data_ids": [],
                "output_data_receivers": [],
                "signer_id": "sweat_welcome.near",
                "signer_public_key": "ed25519:2zZW3xeqAiKCiH1KbcrXWcNdkQXUrMUYHPYkZWiWfxtV"
              }
            },
            "receipt_id": "CzGKh7rF1zoBtqeHhLrK3uy8fhuYBPdPHiEfMqe1hr6V",
            "receiver_id": "sweat_welcome.near"
          }
        },
        {
          "execution_outcome": {
            "block_hash": "34w3v7KtCGd8v9VsE1HXXSWMhPuRwZJdEVYPEUc7JPpL",
            "id": "ueoqZz7ohFbbGCT9hkF6ZJbZyqw3MPdwLi3keCLhqkN",
            "outcome": {
              "executor_id": "sweat_welcome.near",
              "gas_burnt": 223182562500,
              "logs": [],
              "metadata": {
                "gas_profile": null,
                "version": 3
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
                      "deposit": "5130383935839187500"
                    }
                  }
                ],
                "gas_price": "0",
                "input_data_ids": [],
                "output_data_receivers": [],
                "signer_id": "sweat_welcome.near",
                "signer_public_key": "ed25519:2zZW3xeqAiKCiH1KbcrXWcNdkQXUrMUYHPYkZWiWfxtV"
              }
            },
            "receipt_id": "ueoqZz7ohFbbGCT9hkF6ZJbZyqw3MPdwLi3keCLhqkN",
            "receiver_id": "sweat_welcome.near"
          }
        }
      ],
      "transaction": {
        "actions": [
          {
            "FunctionCall": {
              "args": "eyJhY2NvdW50X2lkIjoiZjhlZTc1NGM5YjMxMWJlM2Q4MzhjZjMwNGE5Mzc2ZDZjYTkzMzJiMTg4MWIwNTY5ODRmNzhiMjRhOWQzMjZjYiJ9",
              "deposit": "1250000000000000000000",
              "gas": 30000000000000,
              "method_name": "storage_deposit"
            }
          }
        ],
        "hash": "D2pt9ceGUyiGdJeeZLDd3uBGHF11DmPe4aJFMaoWMmDd",
        "nonce": 64885790392396,
        "public_key": "ed25519:2zZW3xeqAiKCiH1KbcrXWcNdkQXUrMUYHPYkZWiWfxtV",
        "receiver_id": "token.sweat",
        "signature": "ed25519:4oq13Uq9Wrrqy6FtSfKqx4pjUJ2jRSeRQ6yaDNgcDbLGZPstXqw1HKxhEVZ24JHtWy18NeCgh2eiMfGBzmHzDQ95",
        "signer_id": "sweat_welcome.near"
      }
    },
    {
      "data_receipts": [
        {
          "predecessor_id": "token.sweat",
          "receipt": {
            "Data": {
              "data": "",
              "data_id": "D4yvigdBjzYU1g2N9hvnzkeV7MCyepnLnRETVtDu9YnV"
            }
          },
          "receipt_id": "DKguTtMeWCvxyrLPGU9x1rCk5x8msLa7tJM9Pp5AEL1A",
          "receiver_id": "staking-pool.sweatmint.near"
        }
      ],
      "execution_outcome": {
        "block_hash": "8dB8ZPYrnnHrYjmNSavK7Gx2UyHLdAkc5jeFmurku5jh",
        "id": "HV42hanyDVK3MYoW8c17Ufxw83htNEQGk93bzyQRocvS",
        "outcome": {
          "executor_id": "lenguyenwin.near",
          "gas_burnt": 2428019381096,
          "logs": [],
          "metadata": {
            "gas_profile": null,
            "version": 1
          },
          "receipt_ids": [
            "H6Roj3A2SNn7HJXdigLPDHiz2kcob4MHwLg3nnDEH2VZ"
          ],
          "status": {
            "SuccessReceiptId": "H6Roj3A2SNn7HJXdigLPDHiz2kcob4MHwLg3nnDEH2VZ"
          },
          "tokens_burnt": "242801938109600000000"
        },
        "proof": []
      },
      "receipts": [
        {
          "execution_outcome": {
            "block_hash": "aH1RHExfKutiQAwbAM5cXzqRhzzsyntQsv9PD6ZtfLN",
            "id": "H6Roj3A2SNn7HJXdigLPDHiz2kcob4MHwLg3nnDEH2VZ",
            "outcome": {
              "executor_id": "staking-pool.sweatmint.near",
              "gas_burnt": 8835809708062,
              "logs": [],
              "metadata": {
                "gas_profile": null,
                "version": 3
              },
              "receipt_ids": [
                "6f5eNwEKRtATMmmFS86h3iE9PbbY9Zh9Nan9Hyd7fxq",
                "2fWzi7JoFtqnNNZYuWe34STD8FVRGFCCSsZJbZDHGjqK",
                "DZFhUY1Yv4RLz7jS6sk6Wkf8Xv7yLUUBiwQYA8oLrKZU"
              ],
              "status": {
                "SuccessReceiptId": "2fWzi7JoFtqnNNZYuWe34STD8FVRGFCCSsZJbZDHGjqK"
              },
              "tokens_burnt": "883580970806200000000"
            },
            "proof": []
          },
          "receipt": {
            "predecessor_id": "lenguyenwin.near",
            "receipt": {
              "Action": {
                "actions": [
                  {
                    "FunctionCall": {
                      "args": "eyJzdGFraW5nX3BhY2thZ2VfaWQiOjQ4Nn0=",
                      "deposit": "1",
                      "gas": 60000000000000,
                      "method_name": "claim_stake_reward"
                    }
                  }
                ],
                "gas_price": "146853372",
                "input_data_ids": [],
                "output_data_receivers": [],
                "signer_id": "lenguyenwin.near",
                "signer_public_key": "ed25519:4iW7dw89iZrYNbbewyZu2DhuRcpfiLSCqut9aT5x79B1"
              }
            },
            "receipt_id": "H6Roj3A2SNn7HJXdigLPDHiz2kcob4MHwLg3nnDEH2VZ",
            "receiver_id": "staking-pool.sweatmint.near"
          }
        },
        {
          "execution_outcome": {
            "block_hash": "4uFiEE1RYwYV4FBrJb75dTuVxeYSkWuHRG33SRkBNYP1",
            "id": "6f5eNwEKRtATMmmFS86h3iE9PbbY9Zh9Nan9Hyd7fxq",
            "outcome": {
              "executor_id": "reward-sweatmint.near",
              "gas_burnt": 5260616890821,
              "logs": [],
              "metadata": {
                "gas_profile": null,
                "version": 3
              },
              "receipt_ids": [
                "ADVAe56SmMX5c3zsfqR4wpRbCdod9qbWmtsYD2rqVxdh",
                "DLiHiBJRKtZZQtZzoqrmX5cK8nnQDjpGU18dwaCQkuSQ"
              ],
              "status": {
                "SuccessReceiptId": "ADVAe56SmMX5c3zsfqR4wpRbCdod9qbWmtsYD2rqVxdh"
              },
              "tokens_burnt": "526061689082100000000"
            },
            "proof": []
          },
          "receipt": {
            "predecessor_id": "staking-pool.sweatmint.near",
            "receipt": {
              "Action": {
                "actions": [
                  {
                    "FunctionCall": {
                      "args": "eyJyZWNlaXZlcl9pZCI6Imxlbmd1eWVud2luLm5lYXIiLCJhbW91bnQiOiIxMTkxOTgzNTEwOTU4OTA0MTA5NTg5IiwibWVtbyI6IlNUQUtJTkcgQ09OVFJBQ1QgUkVMRUFTRSBSRVdBUkQifQ==",
                      "deposit": "1",
                      "gas": 24367913288503,
                      "method_name": "ft_transfer"
                    }
                  }
                ],
                "gas_price": "146853372",
                "input_data_ids": [],
                "output_data_receivers": [
                  {
                    "data_id": "D4yvigdBjzYU1g2N9hvnzkeV7MCyepnLnRETVtDu9YnV",
                    "receiver_id": "staking-pool.sweatmint.near"
                  }
                ],
                "signer_id": "lenguyenwin.near",
                "signer_public_key": "ed25519:4iW7dw89iZrYNbbewyZu2DhuRcpfiLSCqut9aT5x79B1"
              }
            },
            "receipt_id": "6f5eNwEKRtATMmmFS86h3iE9PbbY9Zh9Nan9Hyd7fxq",
            "receiver_id": "reward-sweatmint.near"
          }
        },
        {
          "execution_outcome": {
            "block_hash": "4uFiEE1RYwYV4FBrJb75dTuVxeYSkWuHRG33SRkBNYP1",
            "id": "DZFhUY1Yv4RLz7jS6sk6Wkf8Xv7yLUUBiwQYA8oLrKZU",
            "outcome": {
              "executor_id": "lenguyenwin.near",
              "gas_burnt": 223182562500,
              "logs": [],
              "metadata": {
                "gas_profile": null,
                "version": 3
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
                      "deposit": "413987479173040285064"
                    }
                  }
                ],
                "gas_price": "0",
                "input_data_ids": [],
                "output_data_receivers": [],
                "signer_id": "lenguyenwin.near",
                "signer_public_key": "ed25519:4iW7dw89iZrYNbbewyZu2DhuRcpfiLSCqut9aT5x79B1"
              }
            },
            "receipt_id": "DZFhUY1Yv4RLz7jS6sk6Wkf8Xv7yLUUBiwQYA8oLrKZU",
            "receiver_id": "lenguyenwin.near"
          }
        },
        {
          "execution_outcome": {
            "block_hash": "6kCXbQPKQGeaakpLjeVMNPRYZpgASgStzmp3APtHGigU",
            "id": "ADVAe56SmMX5c3zsfqR4wpRbCdod9qbWmtsYD2rqVxdh",
            "outcome": {
              "executor_id": "token.sweat",
              "gas_burnt": 3440499484455,
              "logs": [
                "EVENT_JSON:{\"standard\":\"nep141\",\"version\":\"1.0.0\",\"event\":\"ft_transfer\",\"data\":[{\"old_owner_id\":\"reward-sweatmint.near\",\"new_owner_id\":\"lenguyenwin.near\",\"amount\":\"1191983510958904109589\",\"memo\":\"STAKING CONTRACT RELEASE REWARD\"}]}"
              ],
              "metadata": {
                "gas_profile": null,
                "version": 3
              },
              "receipt_ids": [
                "FTsGUiWDfTbAXWELi78i7ANWfnCtxtzk3DEdYqWF49uE"
              ],
              "status": {
                "SuccessValue": ""
              },
              "tokens_burnt": "344049948445500000000"
            },
            "proof": []
          },
          "receipt": {
            "predecessor_id": "reward-sweatmint.near",
            "receipt": {
              "Action": {
                "actions": [
                  {
                    "FunctionCall": {
                      "args": "eyJyZWNlaXZlcl9pZCI6Imxlbmd1eWVud2luLm5lYXIiLCJhbW91bnQiOiIxMTkxOTgzNTEwOTU4OTA0MTA5NTg5IiwibWVtbyI6IlNUQUtJTkcgQ09OVFJBQ1QgUkVMRUFTRSBSRVdBUkQifQ==",
                      "deposit": "1",
                      "gas": 19107296397682,
                      "method_name": "ft_transfer"
                    }
                  }
                ],
                "gas_price": "146853372",
                "input_data_ids": [],
                "output_data_receivers": [
                  {
                    "data_id": "D4yvigdBjzYU1g2N9hvnzkeV7MCyepnLnRETVtDu9YnV",
                    "receiver_id": "staking-pool.sweatmint.near"
                  }
                ],
                "signer_id": "lenguyenwin.near",
                "signer_public_key": "ed25519:4iW7dw89iZrYNbbewyZu2DhuRcpfiLSCqut9aT5x79B1"
              }
            },
            "receipt_id": "ADVAe56SmMX5c3zsfqR4wpRbCdod9qbWmtsYD2rqVxdh",
            "receiver_id": "token.sweat"
          }
        },
        {
          "execution_outcome": {
            "block_hash": "6kCXbQPKQGeaakpLjeVMNPRYZpgASgStzmp3APtHGigU",
            "id": "DLiHiBJRKtZZQtZzoqrmX5cK8nnQDjpGU18dwaCQkuSQ",
            "outcome": {
              "executor_id": "lenguyenwin.near",
              "gas_burnt": 223182562500,
              "logs": [],
              "metadata": {
                "gas_profile": null,
                "version": 3
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
                      "deposit": "246477640135119698412"
                    }
                  }
                ],
                "gas_price": "0",
                "input_data_ids": [],
                "output_data_receivers": [],
                "signer_id": "lenguyenwin.near",
                "signer_public_key": "ed25519:4iW7dw89iZrYNbbewyZu2DhuRcpfiLSCqut9aT5x79B1"
              }
            },
            "receipt_id": "DLiHiBJRKtZZQtZzoqrmX5cK8nnQDjpGU18dwaCQkuSQ",
            "receiver_id": "lenguyenwin.near"
          }
        },
        {
          "execution_outcome": {
            "block_hash": "DtnJZKVRLj7UaYTsJ1JU8XqeC4kcGCTFUm8st8HT4iXz",
            "id": "FTsGUiWDfTbAXWELi78i7ANWfnCtxtzk3DEdYqWF49uE",
            "outcome": {
              "executor_id": "lenguyenwin.near",
              "gas_burnt": 223182562500,
              "logs": [],
              "metadata": {
                "gas_profile": null,
                "version": 3
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
                      "deposit": "2818508745690863017464"
                    }
                  }
                ],
                "gas_price": "0",
                "input_data_ids": [],
                "output_data_receivers": [],
                "signer_id": "lenguyenwin.near",
                "signer_public_key": "ed25519:4iW7dw89iZrYNbbewyZu2DhuRcpfiLSCqut9aT5x79B1"
              }
            },
            "receipt_id": "FTsGUiWDfTbAXWELi78i7ANWfnCtxtzk3DEdYqWF49uE",
            "receiver_id": "lenguyenwin.near"
          }
        },
        {
          "execution_outcome": {
            "block_hash": "DtnJZKVRLj7UaYTsJ1JU8XqeC4kcGCTFUm8st8HT4iXz",
            "id": "2fWzi7JoFtqnNNZYuWe34STD8FVRGFCCSsZJbZDHGjqK",
            "outcome": {
              "executor_id": "staking-pool.sweatmint.near",
              "gas_burnt": 3065464793954,
              "logs": [],
              "metadata": {
                "gas_profile": null,
                "version": 3
              },
              "receipt_ids": [
                "75jzbd2doEAP5AMweFMuviBFPq11FCDnEmMFypBNFZjs"
              ],
              "status": {
                "SuccessValue": "IjExOTE5ODM1MTA5NTg5MDQxMDk1ODki"
              },
              "tokens_burnt": "306546479395400000000"
            },
            "proof": []
          },
          "receipt": {
            "predecessor_id": "staking-pool.sweatmint.near",
            "receipt": {
              "Action": {
                "actions": [
                  {
                    "FunctionCall": {
                      "args": "eyJhbW91bnQiOiIxMTkxOTgzNTEwOTU4OTA0MTA5NTg5IiwiYWNjb3VudF9pZCI6Imxlbmd1eWVud2luLm5lYXIiLCJzdGFraW5nX3BhY2thZ2VfaWQiOjQ4Nn0=",
                      "deposit": "0",
                      "gas": 24367913288503,
                      "method_name": "ft_claim_stake_reward_callback"
                    }
                  }
                ],
                "gas_price": "146853372",
                "input_data_ids": [
                  "D4yvigdBjzYU1g2N9hvnzkeV7MCyepnLnRETVtDu9YnV"
                ],
                "output_data_receivers": [],
                "signer_id": "lenguyenwin.near",
                "signer_public_key": "ed25519:4iW7dw89iZrYNbbewyZu2DhuRcpfiLSCqut9aT5x79B1"
              }
            },
            "receipt_id": "2fWzi7JoFtqnNNZYuWe34STD8FVRGFCCSsZJbZDHGjqK",
            "receiver_id": "staking-pool.sweatmint.near"
          }
        },
        {
          "execution_outcome": {
            "block_hash": "4mab4E2u6o5i4fnm4CEhtbKUwYq7h5nRb9yiaMhSzVmL",
            "id": "75jzbd2doEAP5AMweFMuviBFPq11FCDnEmMFypBNFZjs",
            "outcome": {
              "executor_id": "lenguyenwin.near",
              "gas_burnt": 223182562500,
              "logs": [],
              "metadata": {
                "gas_profile": null,
                "version": 3
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
                      "deposit": "3628552200667077654772"
                    }
                  }
                ],
                "gas_price": "0",
                "input_data_ids": [],
                "output_data_receivers": [],
                "signer_id": "lenguyenwin.near",
                "signer_public_key": "ed25519:4iW7dw89iZrYNbbewyZu2DhuRcpfiLSCqut9aT5x79B1"
              }
            },
            "receipt_id": "75jzbd2doEAP5AMweFMuviBFPq11FCDnEmMFypBNFZjs",
            "receiver_id": "lenguyenwin.near"
          }
        }
      ],
      "transaction": {
        "actions": [
          {
            "FunctionCall": {
              "args": "eyJzdGFraW5nX3BhY2thZ2VfaWQiOjQ4Nn0=",
              "deposit": "1",
              "gas": 60000000000000,
              "method_name": "claim_stake_reward"
            }
          }
        ],
        "hash": "HV42hanyDVK3MYoW8c17Ufxw83htNEQGk93bzyQRocvS",
        "nonce": 83145267000161,
        "public_key": "ed25519:4iW7dw89iZrYNbbewyZu2DhuRcpfiLSCqut9aT5x79B1",
        "receiver_id": "staking-pool.sweatmint.near",
        "signature": "ed25519:2SAM6JuqwaJzGoEQ1LxPAKQKG235tnfojFrpTiUQzqkD1rprCZdhBz5wHqYorbrcz1F1mnPFs5Kb2p7K2uxCD5Y3",
        "signer_id": "lenguyenwin.near"
      }
    }
  ]
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
    {
      "account_id": "sweat_welcome.near",
      "signer_id": "sweat_welcome.near",
      "transaction_hash": "BMBNhr7j1WUcPZpHwrQLj2kHH3CHEbJ1fPEHdDeL6SoX",
      "tx_block_height": 96098994,
      "tx_block_timestamp": 1688940091339135477
    },
    // ...
  ],
  "transactions": [
    {
      "data_receipts": [],
      "execution_outcome": {
        "block_hash": "ASDm9EzkkCfT89AtX8XUmv2fyFaAobDUZmnsiueGHsJ8",
        "id": "2bCYEdSzAojeQe8BYqbFRZJxaZS8ZMfCQt7gtDabcN3W",
        "outcome": {
          "executor_id": "sweat_welcome.near",
          "gas_burnt": 2428135649664,
          "logs": [],
          "metadata": {
            "gas_profile": null,
            "version": 1
          },
          "receipt_ids": [
            "HpQmCSFMNd3ZuHCSw7wFLBzpwQEqR9XfY8xDTBey8Aak"
          ],
          "status": {
            "SuccessReceiptId": "HpQmCSFMNd3ZuHCSw7wFLBzpwQEqR9XfY8xDTBey8Aak"
          },
          "tokens_burnt": "242813564966400000000"
        },
        "proof": []
      },
      "receipts": [
        {
          "execution_outcome": {
            "block_hash": "EAz7Hbo8UfdPwQEj2KxxKwjdZT8VUiBmaks9s9dULEc3",
            "id": "HpQmCSFMNd3ZuHCSw7wFLBzpwQEqR9XfY8xDTBey8Aak",
            "outcome": {
              "executor_id": "token.sweat",
              "gas_burnt": 3665220497784,
              "logs": [],
              "metadata": {
                "gas_profile": null,
                "version": 3
              },
              "receipt_ids": [
                "3fkeFS5nvFngzoXkRgvigqWwS59JVgzXKTKNYrQxqAm6",
                "FP5U7ftgTkt3z4V5nm5nkLMVLwLqk4YuCzzkuwBmQnDo"
              ],
              "status": {
                "SuccessValue": "eyJ0b3RhbCI6IjkwMDAwMDAwMDAwMDAwMDAwMDAwMCIsImF2YWlsYWJsZSI6IjAifQ=="
              },
              "tokens_burnt": "366522049778400000000"
            },
            "proof": []
          },
          "receipt": {
            "predecessor_id": "sweat_welcome.near",
            "receipt": {
              "Action": {
                "actions": [
                  {
                    "FunctionCall": {
                      "args": "eyJhY2NvdW50X2lkIjoiYWJjMTc2YmU2ZDYzY2FmN2FlMzNjYWYzNDYzYmU2ZjkxMGI4YmI5ZWZiNmJmZWMzZjI2YTY2NDBhMWI0NmYwNCJ9",
                      "deposit": "1250000000000000000000",
                      "gas": 30000000000000,
                      "method_name": "storage_deposit"
                    }
                  }
                ],
                "gas_price": "122987387",
                "input_data_ids": [],
                "output_data_receivers": [],
                "signer_id": "sweat_welcome.near",
                "signer_public_key": "ed25519:D6cHxv3s9wYiWyhsqzKfqQm6XW4fhGS4Eg97U41v3zbh"
              }
            },
            "receipt_id": "HpQmCSFMNd3ZuHCSw7wFLBzpwQEqR9XfY8xDTBey8Aak",
            "receiver_id": "token.sweat"
          }
        },
        {
          "execution_outcome": {
            "block_hash": "GTGQZZS5yExDmC3QPRJ8N9GaHgPxq6q7Udj3ExHdUrr",
            "id": "3fkeFS5nvFngzoXkRgvigqWwS59JVgzXKTKNYrQxqAm6",
            "outcome": {
              "executor_id": "sweat_welcome.near",
              "gas_burnt": 223182562500,
              "logs": [],
              "metadata": {
                "gas_profile": null,
                "version": 3
              },
              "receipt_ids": [
                "8NrrvYoLBC1tuFvv5DdPYsys4F7N8mNGWKkJ18xkFgkQ"
              ],
              "status": {
                "SuccessValue": ""
              },
              "tokens_burnt": "22318256250000000000"
            },
            "proof": []
          },
          "receipt": {
            "predecessor_id": "token.sweat",
            "receipt": {
              "Action": {
                "actions": [
                  {
                    "Transfer": {
                      "deposit": "350000000000000000000"
                    }
                  }
                ],
                "gas_price": "122987387",
                "input_data_ids": [],
                "output_data_receivers": [],
                "signer_id": "sweat_welcome.near",
                "signer_public_key": "ed25519:D6cHxv3s9wYiWyhsqzKfqQm6XW4fhGS4Eg97U41v3zbh"
              }
            },
            "receipt_id": "3fkeFS5nvFngzoXkRgvigqWwS59JVgzXKTKNYrQxqAm6",
            "receiver_id": "sweat_welcome.near"
          }
        },
        {
          "execution_outcome": {
            "block_hash": "GTGQZZS5yExDmC3QPRJ8N9GaHgPxq6q7Udj3ExHdUrr",
            "id": "FP5U7ftgTkt3z4V5nm5nkLMVLwLqk4YuCzzkuwBmQnDo",
            "outcome": {
              "executor_id": "sweat_welcome.near",
              "gas_burnt": 223182562500,
              "logs": [],
              "metadata": {
                "gas_profile": null,
                "version": 3
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
                      "deposit": "3594280978869483600468"
                    }
                  }
                ],
                "gas_price": "0",
                "input_data_ids": [],
                "output_data_receivers": [],
                "signer_id": "sweat_welcome.near",
                "signer_public_key": "ed25519:D6cHxv3s9wYiWyhsqzKfqQm6XW4fhGS4Eg97U41v3zbh"
              }
            },
            "receipt_id": "FP5U7ftgTkt3z4V5nm5nkLMVLwLqk4YuCzzkuwBmQnDo",
            "receiver_id": "sweat_welcome.near"
          }
        },
        {
          "execution_outcome": {
            "block_hash": "7oqPQBBwJPPivVXH9oyyW9yxxS2Za7noajHsvtUMvR5q",
            "id": "8NrrvYoLBC1tuFvv5DdPYsys4F7N8mNGWKkJ18xkFgkQ",
            "outcome": {
              "executor_id": "sweat_welcome.near",
              "gas_burnt": 223182562500,
              "logs": [],
              "metadata": {
                "gas_profile": null,
                "version": 3
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
                      "deposit": "5130383935839187500"
                    }
                  }
                ],
                "gas_price": "0",
                "input_data_ids": [],
                "output_data_receivers": [],
                "signer_id": "sweat_welcome.near",
                "signer_public_key": "ed25519:D6cHxv3s9wYiWyhsqzKfqQm6XW4fhGS4Eg97U41v3zbh"
              }
            },
            "receipt_id": "8NrrvYoLBC1tuFvv5DdPYsys4F7N8mNGWKkJ18xkFgkQ",
            "receiver_id": "sweat_welcome.near"
          }
        }
      ],
      "transaction": {
        "actions": [
          {
            "FunctionCall": {
              "args": "eyJhY2NvdW50X2lkIjoiYWJjMTc2YmU2ZDYzY2FmN2FlMzNjYWYzNDYzYmU2ZjkxMGI4YmI5ZWZiNmJmZWMzZjI2YTY2NDBhMWI0NmYwNCJ9",
              "deposit": "1250000000000000000000",
              "gas": 30000000000000,
              "method_name": "storage_deposit"
            }
          }
        ],
        "hash": "2bCYEdSzAojeQe8BYqbFRZJxaZS8ZMfCQt7gtDabcN3W",
        "nonce": 64885790401249,
        "public_key": "ed25519:D6cHxv3s9wYiWyhsqzKfqQm6XW4fhGS4Eg97U41v3zbh",
        "receiver_id": "token.sweat",
        "signature": "ed25519:4gLc8cJ9TiKWLkSr77sjt8cjy3xouC7LfENufWAAkLAyKWSKvq31JTBKPDv8PaaMnhLMyvMUss8PpYd3KNEk6bns",
        "signer_id": "sweat_welcome.near"
      }
    },
    {
      // ...
    },
    // ...
    {
      // ...
    }
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

```json
{
  "account_txs": [
    {
      "account_id": "near",
      "signer_id": "near",
      "transaction_hash": "2hprGWVmVhQ2uq2Yda7CPTuqG7vJLrUm1GNTM5A4xGoQ",
      "tx_block_height": 9823941,
      "tx_block_timestamp": 1595371606784154928
    },
    {
      "account_id": "near",
      "signer_id": "near",
      "transaction_hash": "6ZSgszcR2kmy8tZ3NgxsTJVUGMBpMhGQE8GZiJt9cS72",
      "tx_block_height": 9823886,
      "tx_block_timestamp": 1595371564186485740
    },
    {
      "account_id": "near",
      "signer_id": "near",
      "transaction_hash": "FRBTkEzjLEQekWmKzKfHoTQQJH6Fi5EKxuACWvjBP9wN",
      "tx_block_height": 9823759,
      "tx_block_timestamp": 1595371466422791521
    },
    {
      "account_id": "near",
      "signer_id": "near",
      "transaction_hash": "2UjjDqfNCTmmBHoTMVCYbSBHiNcD3U5p2Ub8oPKMuggT",
      "tx_block_height": 9823708,
      "tx_block_timestamp": 1595371426556758931
    },
    {
      "account_id": "near",
      "signer_id": "near",
      "transaction_hash": "TUVnGuUaBt15eUoL3gWa4HJc9wDeutHKSHELDBxFeCT",
      "tx_block_height": 9823544,
      "tx_block_timestamp": 1595371300119366393
    },
    {
      "account_id": "near",
      "signer_id": "near",
      "transaction_hash": "9vk1mYUF9axoN4xDPELcPnKf8pJjSn2wEgbof6GHCvx5",
      "tx_block_height": 9823464,
      "tx_block_timestamp": 1595371238536605515
    },
    {
      "account_id": "near",
      "signer_id": "near",
      "transaction_hash": "4QPkJVNQZPjRq1gpnzXqLzXzRDskPGub9F5zyWTHQHkv",
      "tx_block_height": 9823296,
      "tx_block_timestamp": 1595371107851534133
    },
    {
      "account_id": "near",
      "signer_id": "near",
      "transaction_hash": "G7B6QNdRMUg6NrELiBJA7NComt7vrsCXdRPq4h845nE3",
      "tx_block_height": 9823155,
      "tx_block_timestamp": 1595370998735096995
    },
    {
      "account_id": "near",
      "signer_id": "near",
      "transaction_hash": "6ASygFLQDnuXpybTRFPTMYNjLNmpAzRgTDX55YjnpEBz",
      "tx_block_height": 9823032,
      "tx_block_timestamp": 1595370902703624569
    }
  ],
  "transactions": [
    {
      "data_receipts": [],
      "execution_outcome": {
        "block_hash": "G7QEoVLpuzbsGK4S3oCWw55vGK1tgn6E6EXA8NJpxUmN",
        "id": "2UjjDqfNCTmmBHoTMVCYbSBHiNcD3U5p2Ub8oPKMuggT",
        "outcome": {
          "executor_id": "near",
          "gas_burnt": 424555062500,
          "logs": [],
          "metadata": {
            "gas_profile": null,
            "version": 1
          },
          "receipt_ids": [
            "E1wjqVz8geRk3Kc4aQ9s9dgZNHLd1bDAHP5HpvMBw6ei"
          ],
          "status": {
            "SuccessReceiptId": "E1wjqVz8geRk3Kc4aQ9s9dgZNHLd1bDAHP5HpvMBw6ei"
          },
          "tokens_burnt": "424555062500000000000"
        },
        "proof": []
      },
      "receipts": [
        {
          "execution_outcome": {
            "block_hash": "Evg2rJdWcfWoyJX7DN7cxqqARMmy38bj55UDE2aGPY5X",
            "id": "E1wjqVz8geRk3Kc4aQ9s9dgZNHLd1bDAHP5HpvMBw6ei",
            "outcome": {
              "executor_id": "blaze.near",
              "gas_burnt": 424555062500,
              "logs": [],
              "metadata": {
                "gas_profile": null,
                "version": 1
              },
              "receipt_ids": [
                "GyQvZ9snHP6UgdPQNPkDcYdXtUaTsiNijXeaQKzezen2"
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
                      "public_key": "ed25519:3ND4bV4tsYyCt8UjdpcwU6x7vXaeBLRQUPeG1VtBkCTn"
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
            "receipt_id": "E1wjqVz8geRk3Kc4aQ9s9dgZNHLd1bDAHP5HpvMBw6ei",
            "receiver_id": "blaze.near"
          }
        },
        {
          "execution_outcome": {
            "block_hash": "4z3qgmLrAWU7gZ5A7m1Y34tF6Qjzz9BdQKu7WAEqurp5",
            "id": "GyQvZ9snHP6UgdPQNPkDcYdXtUaTsiNijXeaQKzezen2",
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
            "receipt_id": "GyQvZ9snHP6UgdPQNPkDcYdXtUaTsiNijXeaQKzezen2",
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
              "public_key": "ed25519:3ND4bV4tsYyCt8UjdpcwU6x7vXaeBLRQUPeG1VtBkCTn"
            }
          }
        ],
        "hash": "2UjjDqfNCTmmBHoTMVCYbSBHiNcD3U5p2Ub8oPKMuggT",
        "nonce": 14,
        "public_key": "ed25519:5zset1JX4qp4PcR3N9KDSY6ATdgkrbBW5wFBGWC4ZjnU",
        "receiver_id": "blaze.near",
        "signature": "ed25519:5iTL22vu4CaC9Hhr5rpAdsAnGhUodjpNNFCjEknPnrHQiuBMBdPFhRysjSgNjFPvQDKwH98V1heLwpaQvXE2exdf",
        "signer_id": "near"
      }
    },
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
    },
    {
      "data_receipts": [],
      "execution_outcome": {
        "block_hash": "FXC7wy2KK3MHLtJzcnC8m1ZmJmMYH5Rhbz5Zd9BXVZL8",
        "id": "4QPkJVNQZPjRq1gpnzXqLzXzRDskPGub9F5zyWTHQHkv",
        "outcome": {
          "executor_id": "near",
          "gas_burnt": 424555062500,
          "logs": [],
          "metadata": {
            "gas_profile": null,
            "version": 1
          },
          "receipt_ids": [
            "J3JjXgkupUMX7ZG8m542YyaHJVpZhqUrY5g2QCT8egob"
          ],
          "status": {
            "SuccessReceiptId": "J3JjXgkupUMX7ZG8m542YyaHJVpZhqUrY5g2QCT8egob"
          },
          "tokens_burnt": "424555062500000000000"
        },
        "proof": []
      },
      "receipts": [
        {
          "execution_outcome": {
            "block_hash": "AQLkuZV9BPCyGiDXqjYB41di8nBfwwu2P4ZJdm9oTgxB",
            "id": "J3JjXgkupUMX7ZG8m542YyaHJVpZhqUrY5g2QCT8egob",
            "outcome": {
              "executor_id": "ino.near",
              "gas_burnt": 424555062500,
              "logs": [],
              "metadata": {
                "gas_profile": null,
                "version": 1
              },
              "receipt_ids": [
                "5YNBWzoht2pxkgBjuna57bbi6v9v9kdJxdqxjpKcupiq"
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
                      "public_key": "ed25519:59D8uiJ2pQywPmoxTg25kPhwLrbmVohBeQCeY9jug66U"
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
            "receipt_id": "J3JjXgkupUMX7ZG8m542YyaHJVpZhqUrY5g2QCT8egob",
            "receiver_id": "ino.near"
          }
        },
        {
          "execution_outcome": {
            "block_hash": "3PcUDFvmFpaR4kG87RR4kyBTHfte6Rtmawd3YuAP1oJD",
            "id": "5YNBWzoht2pxkgBjuna57bbi6v9v9kdJxdqxjpKcupiq",
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
            "receipt_id": "5YNBWzoht2pxkgBjuna57bbi6v9v9kdJxdqxjpKcupiq",
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
              "public_key": "ed25519:59D8uiJ2pQywPmoxTg25kPhwLrbmVohBeQCeY9jug66U"
            }
          }
        ],
        "hash": "4QPkJVNQZPjRq1gpnzXqLzXzRDskPGub9F5zyWTHQHkv",
        "nonce": 11,
        "public_key": "ed25519:5zset1JX4qp4PcR3N9KDSY6ATdgkrbBW5wFBGWC4ZjnU",
        "receiver_id": "ino.near",
        "signature": "ed25519:4EZuUKFgLjzCPrik5JqwCYMi7CpDvWaK2bHsvqDV6mrnsj2mQqexeaZ5RxRfen9pzUEC7uifATLSEMaZoB8oCvYH",
        "signer_id": "near"
      }
    },
    {
      "data_receipts": [],
      "execution_outcome": {
        "block_hash": "FNszSgF84SB341GcEfvo3aA2tX2qK5H3c5Q8xHmu6isF",
        "id": "6ASygFLQDnuXpybTRFPTMYNjLNmpAzRgTDX55YjnpEBz",
        "outcome": {
          "executor_id": "near",
          "gas_burnt": 424555062500,
          "logs": [],
          "metadata": {
            "gas_profile": null,
            "version": 1
          },
          "receipt_ids": [
            "EgNre6vjcPhMBZXCK6ApPYCxSnpCJA3VRqSFTtotz5BH"
          ],
          "status": {
            "SuccessReceiptId": "EgNre6vjcPhMBZXCK6ApPYCxSnpCJA3VRqSFTtotz5BH"
          },
          "tokens_burnt": "424555062500000000000"
        },
        "proof": []
      },
      "receipts": [
        {
          "execution_outcome": {
            "block_hash": "4YM3tuxck9uHaqzr9cB5XSjup6C6oTWMsKL5YvQeoXhd",
            "id": "EgNre6vjcPhMBZXCK6ApPYCxSnpCJA3VRqSFTtotz5BH",
            "outcome": {
              "executor_id": "erm.near",
              "gas_burnt": 424555062500,
              "logs": [],
              "metadata": {
                "gas_profile": null,
                "version": 1
              },
              "receipt_ids": [
                "8uuXJSLCqa83tcfpykZ8pT9mCVHfKFQnktpC8oNfuJUw"
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
                      "public_key": "ed25519:7X68v5JLYa2gEu5N4K2dRLNRranoMC45df7SotoB2mXX"
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
            "receipt_id": "EgNre6vjcPhMBZXCK6ApPYCxSnpCJA3VRqSFTtotz5BH",
            "receiver_id": "erm.near"
          }
        },
        {
          "execution_outcome": {
            "block_hash": "8X6PhDKXggGvfJfDisjJv8fjSCMHxrj4VfMPm3Wt7oyW",
            "id": "8uuXJSLCqa83tcfpykZ8pT9mCVHfKFQnktpC8oNfuJUw",
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
            "receipt_id": "8uuXJSLCqa83tcfpykZ8pT9mCVHfKFQnktpC8oNfuJUw",
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
              "public_key": "ed25519:7X68v5JLYa2gEu5N4K2dRLNRranoMC45df7SotoB2mXX"
            }
          }
        ],
        "hash": "6ASygFLQDnuXpybTRFPTMYNjLNmpAzRgTDX55YjnpEBz",
        "nonce": 9,
        "public_key": "ed25519:5zset1JX4qp4PcR3N9KDSY6ATdgkrbBW5wFBGWC4ZjnU",
        "receiver_id": "erm.near",
        "signature": "ed25519:2dyPpoesYrcddTDvYFH6J3xfkqMyRj3SKZXvEkTvWqX4rMuNVSwtZD96VCtW3hViUbZ1ohkgBfAdrhs82AnGuC7q",
        "signer_id": "near"
      }
    },
    {
      "data_receipts": [],
      "execution_outcome": {
        "block_hash": "DC3o8YH7dpUGttHAx1S6qkLYrAW92t96x9RMTUW17U1x",
        "id": "6ZSgszcR2kmy8tZ3NgxsTJVUGMBpMhGQE8GZiJt9cS72",
        "outcome": {
          "executor_id": "near",
          "gas_burnt": 424555062500,
          "logs": [],
          "metadata": {
            "gas_profile": null,
            "version": 1
          },
          "receipt_ids": [
            "93muJFiSwoWXGCCK9yxsN8HNwhz5X6egUjyY8hHR6Hyg"
          ],
          "status": {
            "SuccessReceiptId": "93muJFiSwoWXGCCK9yxsN8HNwhz5X6egUjyY8hHR6Hyg"
          },
          "tokens_burnt": "424555062500000000000"
        },
        "proof": []
      },
      "receipts": [
        {
          "execution_outcome": {
            "block_hash": "AimtVeHYLH3DygPZx4teWiXNc4xGe3kbav58HNipGA1Z",
            "id": "93muJFiSwoWXGCCK9yxsN8HNwhz5X6egUjyY8hHR6Hyg",
            "outcome": {
              "executor_id": "gaia.near",
              "gas_burnt": 424555062500,
              "logs": [],
              "metadata": {
                "gas_profile": null,
                "version": 1
              },
              "receipt_ids": [
                "G61h4exS67bYy5ZZsA61sEw39bPwfpaJKUMGYVbbsKrV"
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
                      "public_key": "ed25519:DUnYUTBtzHqUvKUim2ZXRHJVtwbjyoU2VqGS1dLu8baf"
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
            "receipt_id": "93muJFiSwoWXGCCK9yxsN8HNwhz5X6egUjyY8hHR6Hyg",
            "receiver_id": "gaia.near"
          }
        },
        {
          "execution_outcome": {
            "block_hash": "9zjrHttKzd7cCEZgpQ4Hf3xzpZnym4E5PT8t8sr9CZE5",
            "id": "G61h4exS67bYy5ZZsA61sEw39bPwfpaJKUMGYVbbsKrV",
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
            "receipt_id": "G61h4exS67bYy5ZZsA61sEw39bPwfpaJKUMGYVbbsKrV",
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
              "public_key": "ed25519:DUnYUTBtzHqUvKUim2ZXRHJVtwbjyoU2VqGS1dLu8baf"
            }
          }
        ],
        "hash": "6ZSgszcR2kmy8tZ3NgxsTJVUGMBpMhGQE8GZiJt9cS72",
        "nonce": 16,
        "public_key": "ed25519:5zset1JX4qp4PcR3N9KDSY6ATdgkrbBW5wFBGWC4ZjnU",
        "receiver_id": "gaia.near",
        "signature": "ed25519:4qiCF1Gijs2byW4oV1MjduxbnEuJ8sHFtkVw8PJqjp6socFVymXSyo7nUHHYUs5QL4QUdzivYbWhiMUpEHkgsq8P",
        "signer_id": "near"
      }
    },
    {
      "data_receipts": [],
      "execution_outcome": {
        "block_hash": "CfQSoyBP7h7rCFkuep2nAkPkgM3ts4JT72ZHKmaqj5eA",
        "id": "9vk1mYUF9axoN4xDPELcPnKf8pJjSn2wEgbof6GHCvx5",
        "outcome": {
          "executor_id": "near",
          "gas_burnt": 424555062500,
          "logs": [],
          "metadata": {
            "gas_profile": null,
            "version": 1
          },
          "receipt_ids": [
            "41ZbG44di5Ady4rq29MGUexXCxSCdNmNwjrhqYK6CerA"
          ],
          "status": {
            "SuccessReceiptId": "41ZbG44di5Ady4rq29MGUexXCxSCdNmNwjrhqYK6CerA"
          },
          "tokens_burnt": "424555062500000000000"
        },
        "proof": []
      },
      "receipts": [
        {
          "execution_outcome": {
            "block_hash": "FaxaeFx1yygzuHwifQBLBJcqNEWjRXFoJiCyWzDcbVmK",
            "id": "41ZbG44di5Ady4rq29MGUexXCxSCdNmNwjrhqYK6CerA",
            "outcome": {
              "executor_id": "dsrvlabs.near",
              "gas_burnt": 424555062500,
              "logs": [],
              "metadata": {
                "gas_profile": null,
                "version": 1
              },
              "receipt_ids": [
                "GP5eoQwqhJiS6q2ojPJwkqCYYuLppwpZ7XFq6bggryJD"
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
                      "public_key": "ed25519:5DeTWhT3rQn1i99gw6BqvjKgAKFGTFkEhzqW2qzkeGwb"
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
            "receipt_id": "41ZbG44di5Ady4rq29MGUexXCxSCdNmNwjrhqYK6CerA",
            "receiver_id": "dsrvlabs.near"
          }
        },
        {
          "execution_outcome": {
            "block_hash": "FJVrM79HJ7EFVSKFz6rQyFaPYT4pmb1VRCFokVY53h4V",
            "id": "GP5eoQwqhJiS6q2ojPJwkqCYYuLppwpZ7XFq6bggryJD",
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
            "receipt_id": "GP5eoQwqhJiS6q2ojPJwkqCYYuLppwpZ7XFq6bggryJD",
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
              "public_key": "ed25519:5DeTWhT3rQn1i99gw6BqvjKgAKFGTFkEhzqW2qzkeGwb"
            }
          }
        ],
        "hash": "9vk1mYUF9axoN4xDPELcPnKf8pJjSn2wEgbof6GHCvx5",
        "nonce": 12,
        "public_key": "ed25519:5zset1JX4qp4PcR3N9KDSY6ATdgkrbBW5wFBGWC4ZjnU",
        "receiver_id": "dsrvlabs.near",
        "signature": "ed25519:2n45YrYUzSn35xS1EqB5LP8gsPY2gg7T35Ngghc1xFDdY2LTC4AqiPUtDnhg1apxhmnrvKNtz7jwJLekrAYJsGbv",
        "signer_id": "near"
      }
    },
    {
      "data_receipts": [],
      "execution_outcome": {
        "block_hash": "aiWXsSZZzQtaj82rq6Lk6sF4nXqi1GMMN2yKHVozwNB",
        "id": "FRBTkEzjLEQekWmKzKfHoTQQJH6Fi5EKxuACWvjBP9wN",
        "outcome": {
          "executor_id": "near",
          "gas_burnt": 424555062500,
          "logs": [],
          "metadata": {
            "gas_profile": null,
            "version": 1
          },
          "receipt_ids": [
            "CyTJV9kcp6eLrLZbX9hDZQLZ7BJKzMpx7kHMWnhL2v8Z"
          ],
          "status": {
            "SuccessReceiptId": "CyTJV9kcp6eLrLZbX9hDZQLZ7BJKzMpx7kHMWnhL2v8Z"
          },
          "tokens_burnt": "424555062500000000000"
        },
        "proof": []
      },
      "receipts": [
        {
          "execution_outcome": {
            "block_hash": "A8VgUb7hrzKJkp3ThnCaZzf4kpKNF1k7XUzVx9tkmtq9",
            "id": "CyTJV9kcp6eLrLZbX9hDZQLZ7BJKzMpx7kHMWnhL2v8Z",
            "outcome": {
              "executor_id": "henry.near",
              "gas_burnt": 424555062500,
              "logs": [],
              "metadata": {
                "gas_profile": null,
                "version": 1
              },
              "receipt_ids": [
                "CJxKndQzphnVdJ2mPjkR9bgK4cxQrsPUd441y9FrdYSv"
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
                      "public_key": "ed25519:HbSrzVndedNoLZFD6FHRwStRntLHWfAprzoSdEF81fLi"
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
            "receipt_id": "CyTJV9kcp6eLrLZbX9hDZQLZ7BJKzMpx7kHMWnhL2v8Z",
            "receiver_id": "henry.near"
          }
        },
        {
          "execution_outcome": {
            "block_hash": "cijUBRADeZLCFU64RTJpMTdx7DdUoFcEKgc56Zrt9xV",
            "id": "CJxKndQzphnVdJ2mPjkR9bgK4cxQrsPUd441y9FrdYSv",
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
            "receipt_id": "CJxKndQzphnVdJ2mPjkR9bgK4cxQrsPUd441y9FrdYSv",
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
              "public_key": "ed25519:HbSrzVndedNoLZFD6FHRwStRntLHWfAprzoSdEF81fLi"
            }
          }
        ],
        "hash": "FRBTkEzjLEQekWmKzKfHoTQQJH6Fi5EKxuACWvjBP9wN",
        "nonce": 15,
        "public_key": "ed25519:5zset1JX4qp4PcR3N9KDSY6ATdgkrbBW5wFBGWC4ZjnU",
        "receiver_id": "henry.near",
        "signature": "ed25519:4DSnVuRz6HmdHXZvzXQRqMA5xXBrY8SBdPhbcuc4pPnzenxSkBs9F3dUzwr62LeabsyoFBMpHQWxdi9Kqhfb9adQ",
        "signer_id": "near"
      }
    },
    {
      "data_receipts": [],
      "execution_outcome": {
        "block_hash": "7Y1BiUQbQjpbvEkVK3RYFWD1psez74afhZnieDWQ9QYx",
        "id": "G7B6QNdRMUg6NrELiBJA7NComt7vrsCXdRPq4h845nE3",
        "outcome": {
          "executor_id": "near",
          "gas_burnt": 424555062500,
          "logs": [],
          "metadata": {
            "gas_profile": null,
            "version": 1
          },
          "receipt_ids": [
            "FFUVsmMjuuniCpEpQ87khvk4xZ1UTrh7cDyJSfBTBj33"
          ],
          "status": {
            "SuccessReceiptId": "FFUVsmMjuuniCpEpQ87khvk4xZ1UTrh7cDyJSfBTBj33"
          },
          "tokens_burnt": "424555062500000000000"
        },
        "proof": []
      },
      "receipts": [
        {
          "execution_outcome": {
            "block_hash": "CxGeqeUfkHvgXifMgSKK7TppoX6QRwrxkse2WNxZKJFc",
            "id": "FFUVsmMjuuniCpEpQ87khvk4xZ1UTrh7cDyJSfBTBj33",
            "outcome": {
              "executor_id": "inotel.near",
              "gas_burnt": 424555062500,
              "logs": [],
              "metadata": {
                "gas_profile": null,
                "version": 1
              },
              "receipt_ids": [
                "FshQB3mrcsYmjAXwwSQ1CHiS7Nk8bHikKSLSZ9QQJPpJ"
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
                      "public_key": "ed25519:59D8uiJ2pQywPmoxTg25kPhwLrbmVohBeQCeY9jug66U"
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
            "receipt_id": "FFUVsmMjuuniCpEpQ87khvk4xZ1UTrh7cDyJSfBTBj33",
            "receiver_id": "inotel.near"
          }
        },
        {
          "execution_outcome": {
            "block_hash": "QcWQCMJw1QE1KGPpkffXMFWYrc3jf7D6asCJSQq7cRo",
            "id": "FshQB3mrcsYmjAXwwSQ1CHiS7Nk8bHikKSLSZ9QQJPpJ",
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
            "receipt_id": "FshQB3mrcsYmjAXwwSQ1CHiS7Nk8bHikKSLSZ9QQJPpJ",
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
              "public_key": "ed25519:59D8uiJ2pQywPmoxTg25kPhwLrbmVohBeQCeY9jug66U"
            }
          }
        ],
        "hash": "G7B6QNdRMUg6NrELiBJA7NComt7vrsCXdRPq4h845nE3",
        "nonce": 10,
        "public_key": "ed25519:5zset1JX4qp4PcR3N9KDSY6ATdgkrbBW5wFBGWC4ZjnU",
        "receiver_id": "inotel.near",
        "signature": "ed25519:2t8FUdzUBC5d4j2qTfN7UbyfU3DNDK2emVogqoj62mrtf7J5nkaoSjYDMTZAUv94Ap7LwpT6cPwmCmMbDs2aBTRr",
        "signer_id": "near"
      }
    },
    {
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
