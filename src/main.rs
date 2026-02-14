mod api;
mod cache;
mod click;
mod types;

use dotenv::dotenv;
use std::env;

use crate::click::ClickDB;
use crate::types::*;
use actix_cors::Cors;
use actix_web::http::header;
use actix_web::{middleware, web, App, HttpResponse, HttpServer, Responder};
use near_primitives::hash::CryptoHash;
use near_primitives::types::{AccountId, BlockHeight};
use tracing_subscriber::EnvFilter;

const PROJECT_ID: &str = "server";

#[derive(Clone)]
pub struct AppState {
    pub click_db: ClickDB,
}

async fn index() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../static/index.html"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    openssl_probe::init_ssl_cert_env_vars();
    dotenv().ok();

    tracing_subscriber::fmt::Subscriber::builder()
        .with_env_filter(EnvFilter::from_default_env())
        // .with_env_filter(EnvFilter::new("debug"))
        .with_writer(std::io::stderr)
        .init();

    let click_db = ClickDB::new();
    click_db
        .verify_connection()
        .await
        .expect("Failed to connect to Clickhouse");

    let bind_address = format!("127.0.0.1:{}", env::var("PORT").unwrap());
    tracing::info!(target: PROJECT_ID, "Listening on {}", bind_address);

    HttpServer::new(move || {
        // Configure CORS middleware
        let cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
                header::ACCEPT,
            ])
            .max_age(3600)
            .supports_credentials();

        let api_v0 = web::scope("/v0")
            .service(api::v0::get_transactions)
            .service(api::v0::get_account)
            .service(api::v0::get_block)
            .service(api::v0::get_blocks)
            .service(api::v0::get_receipt);
        App::new()
            .app_data(web::Data::new(AppState {
                click_db: click_db.clone(),
            }))
            .wrap(cors)
            .wrap(middleware::Logger::new(
                "%{r}a \"%r\"	%s %b \"%{Referer}i\" \"%{User-Agent}i\" %T",
            ))
            .wrap(tracing_actix_web::TracingLogger::default())
            .service(api_v0)
            .route("/", web::get().to(index))
    })
    .bind(bind_address)?
    .run()
    .await?;

    Ok(())
}
