use dotenv::dotenv;
use std::env;

use actix_cors::Cors;
use actix_web::http::header;
use actix_web::{middleware, web, App, HttpServer};
use explorer_api::{api_v0_scope, click::ClickDB, health, index, skill_md, status, AppState};
use tracing_subscriber::EnvFilter;

const PROJECT_ID: &str = "server";

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

    // Bound outside the `HttpServer::new` closure: that closure runs once per worker
    // thread, so building these inside it would give every worker its own start time.
    let started_at = std::time::Instant::now();
    let max_lag_seconds = env::var("HEALTH_MAX_LAG_SECONDS")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(explorer_api::api::DEFAULT_MAX_LAG_SECONDS);

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

        App::new()
            .app_data(web::Data::new(AppState {
                click_db: click_db.clone(),
                started_at,
                max_lag_seconds,
            }))
            .wrap(cors)
            .wrap(middleware::Logger::new(
                "%{r}a \"%r\"	%s %b \"%{Referer}i\" \"%{User-Agent}i\" %T",
            ))
            .wrap(tracing_actix_web::TracingLogger::default())
            .service(api_v0_scope())
            .route("/", web::get().to(index))
            .route("/skill.md", web::get().to(skill_md))
            .route("/health", web::get().to(health))
            .route("/status", web::get().to(status))
    })
    .bind(bind_address)?
    .run()
    .await?;

    Ok(())
}
