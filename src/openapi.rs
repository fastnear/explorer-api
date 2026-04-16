use std::path::PathBuf;

use anyhow::Result;
use fastnear_openapi_generator::{
    build_service_doc, write_or_check_yaml, AggregateOperationSpec, ApiInfo, ApiServer,
    HttpMethod, RequestBodySpec, ResponseContent, ResponseSpec, SchemaRegistry,
};
use schemars::JsonSchema;
use serde_json::{json, Value};

use crate::types::{
    AccountInput, AccountResponse, ApiError, BlockInput, BlockResponse, BlocksInput,
    BlocksResponse, ReceiptInput, ReceiptResponse, TransactionsResponse, TxInput,
};

const API_VERSION: &str = "3.0.3";
const SERVICE_INFO: ApiInfo<'static> = ApiInfo {
    title: "Transactions API",
    version: API_VERSION,
    description: "Transaction, receipt, block, and account-history queries backed by ClickHouse.",
    servers: &[
        ApiServer {
            url: "https://tx.main.fastnear.com",
            description: "Mainnet",
        },
        ApiServer {
            url: "https://tx.test.fastnear.com",
            description: "Testnet",
        },
    ],
};

pub fn generate(check: bool) -> Result<()> {
    let output_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("openapi");

    let mut registry = SchemaRegistry::openapi3();
    let transactions = post_operation_spec::<TxInput, TransactionsResponse>(
        &mut registry,
        "transactions",
        "Transactions API - Transactions by Hash",
        "/v0/transactions",
        "get_transactions",
        "Fetch transactions by hash",
        "Use this endpoint to fetch up to 20 indexed transactions by hash.",
        &["transactions"],
        json!({
            "tx_hashes": [
                "HV42hanyDVK3MYoW8c17Ufxw83htNEQGk93bzyQRocvS",
                "D2pt9ceGUyiGdJeeZLDd3uBGHF11DmPe4aJFMaoWMmDd"
            ]
        }),
        "Matching transactions",
    );
    let account = post_operation_spec::<AccountInput, AccountResponse>(
        &mut registry,
        "account",
        "Transactions API - Account History",
        "/v0/account",
        "get_account",
        "Fetch account transaction history",
        "Fetch account transaction history with optional filters for signer, success status, and sort order.",
        &["account"],
        json!({
            "account_id": "intents.near",
            "is_real_signer": true,
            "is_success": true,
            "limit": 50,
            "desc": true
        }),
        "Account history result set",
    );
    let block = post_operation_spec::<BlockInput, BlockResponse>(
        &mut registry,
        "block",
        "Transactions API - Block Lookup",
        "/v0/block",
        "get_block",
        "Fetch a block by height or hash",
        "Fetch a block row by height or hash and optionally expand its transactions and receipts.",
        &["blocks"],
        json!({
            "block_id": 130000000,
            "with_transactions": true,
            "with_receipts": true
        }),
        "Block lookup result",
    );
    let blocks = post_operation_spec::<BlocksInput, BlocksResponse>(
        &mut registry,
        "blocks",
        "Transactions API - Block Range",
        "/v0/blocks",
        "get_blocks",
        "Fetch a list of blocks",
        "Fetch a bounded list of indexed blocks ordered ascending or descending.",
        &["blocks"],
        json!({
            "from_block_height": 130000000,
            "to_block_height": 130000100,
            "limit": 10,
            "desc": false
        }),
        "Block rows",
    );
    let receipt = post_operation_spec::<ReceiptInput, ReceiptResponse>(
        &mut registry,
        "receipt",
        "Transactions API - Receipt Lookup",
        "/v0/receipt",
        "get_receipt",
        "Fetch a receipt by ID",
        "Use this endpoint to fetch a receipt row and the associated transaction when available.",
        &["receipts"],
        json!({
            "receipt_id": "H6Roj3A2SNn7HJXdigLPDHiz2kcob4MHwLg3nnDEH2VZ"
        }),
        "Receipt lookup result",
    );
    let components = registry.into_components();

    let service_doc = build_service_doc(
        &SERVICE_INFO,
        vec![transactions?, account?, block?, blocks?, receipt?],
        components,
    );

    write_or_check_yaml(output_root.join("openapi.yaml"), &service_doc, check)?;
    Ok(())
}

fn post_operation_spec<Request, Response>(
    registry: &mut SchemaRegistry,
    slug: &'static str,
    title: &'static str,
    path: &'static str,
    operation_id: &'static str,
    summary: &'static str,
    description: &'static str,
    tags: &'static [&'static str],
    request_example: Value,
    ok_description: &'static str,
) -> Result<AggregateOperationSpec<'static>>
where
    Request: JsonSchema,
    Response: JsonSchema,
{
    let request = registry.schema_ref::<Request>();
    let response = registry.schema_ref::<Response>();
    let api_error = registry.schema_ref::<ApiError>();

    Ok(AggregateOperationSpec {
        slug,
        title,
        path,
        method: HttpMethod::Post,
        operation_id,
        summary,
        description,
        tags,
        parameters: vec![],
        request_body: Some(RequestBodySpec::Json {
            schema: request,
            required: true,
            example: Some(request_example),
            examples: vec![],
        }),
        responses: vec![
            ResponseSpec {
                status: "200",
                description: ok_description,
                content: Some(ResponseContent::Json {
                    schema: response,
                    example: None,
                    examples: vec![],
                }),
            },
            ResponseSpec {
                status: "400",
                description: "Bad request",
                content: Some(ResponseContent::Json {
                    schema: api_error,
                    example: None,
                    examples: vec![],
                }),
            },
            ResponseSpec {
                status: "500",
                description: "ClickHouse query failure",
                content: Some(ResponseContent::Json {
                    schema: json!({ "type": "string" }),
                    example: None,
                    examples: vec![],
                }),
            },
        ],
    })
}

#[cfg(test)]
mod tests {
    use fastnear_openapi_generator::SchemaRegistry;

    use crate::types::{
        AccountInput, BlockInput, BlockResponse, ReceiptResponse, TransactionsResponse, TxInput,
    };

    #[test]
    fn tx_hashes_schema_uses_string_items_with_limit() {
        let mut registry = SchemaRegistry::openapi3();
        registry.schema_ref::<TxInput>();
        let components = registry.into_components();
        let tx_hashes = &components["TxInput"]["properties"]["tx_hashes"];

        assert_eq!(tx_hashes["type"], "array");
        assert_eq!(tx_hashes["items"]["type"], "string");
        assert_eq!(tx_hashes["maxItems"], 20);
    }

    #[test]
    fn block_id_schema_is_height_or_hash() {
        let mut registry = SchemaRegistry::openapi3();
        registry.schema_ref::<BlockInput>();
        let components = registry.into_components();
        let block_id = &components["BlockInput"]["properties"]["block_id"]["oneOf"];

        assert_eq!(block_id[0]["type"], "integer");
        assert_eq!(block_id[1]["type"], "string");
    }

    #[test]
    fn raw_transaction_fields_use_object_schema() {
        let mut registry = SchemaRegistry::openapi3();
        registry.schema_ref::<TransactionsResponse>();
        registry.schema_ref::<ReceiptResponse>();
        let components = registry.into_components();

        assert_eq!(
            components["TransactionsResponse"]["properties"]["transactions"]["items"]["type"],
            "object"
        );
        assert_eq!(
            components["ReceiptResponse"]["properties"]["transaction"]["type"],
            "object"
        );
        assert_eq!(
            components["ReceiptResponse"]["properties"]["transaction"]["nullable"],
            true
        );
    }

    #[test]
    fn account_resume_token_schema_is_string_and_nullable() {
        let mut registry = SchemaRegistry::openapi3();
        registry.schema_ref::<AccountInput>();
        let components = registry.into_components();
        let resume_token = &components["AccountInput"]["properties"]["resume_token"];

        assert_eq!(resume_token["type"], "string");
        assert_eq!(resume_token["nullable"], true);
    }

    #[test]
    fn nullable_required_fields_survive_without_overlay_patches() {
        let mut registry = SchemaRegistry::openapi3();
        registry.schema_ref::<BlockResponse>();
        registry.schema_ref::<ReceiptResponse>();
        let components = registry.into_components();

        let block_required = components["BlockResponse"]["required"].as_array().unwrap();
        assert!(block_required.iter().any(|value| value == "block"));
        assert_eq!(
            components["BlockResponse"]["properties"]["block"]["nullable"],
            true
        );

        let receipt_required = components["ReceiptResponse"]["required"]
            .as_array()
            .unwrap();
        assert!(receipt_required.iter().any(|value| value == "receipt"));
        assert!(receipt_required.iter().any(|value| value == "transaction"));
        assert_eq!(
            components["ReceiptResponse"]["properties"]["receipt"]["nullable"],
            true
        );
        assert_eq!(
            components["ReceiptResponse"]["properties"]["transaction"]["nullable"],
            true
        );
    }
}
