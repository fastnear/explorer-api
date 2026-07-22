use dotenv::dotenv;
use std::env;

use actix_cors::Cors;
use actix_web::http::header;
use actix_web::{middleware, web, App, HttpServer};
use explorer_api::{api_v0_scope, click::ClickDB, index, skill_md, AppState};
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
            }))
            .wrap(cors)
            .wrap(middleware::Logger::new(
                "%{r}a \"%r\"	%s %b \"%{Referer}i\" \"%{User-Agent}i\" %T",
            ))
            .wrap(tracing_actix_web::TracingLogger::default())
            .service(api_v0_scope())
            .route("/", web::get().to(index))
            .route("/skill.md", web::get().to(skill_md))
    })
    .bind(bind_address)?
    .run()
    .await?;

    Ok(())
}
