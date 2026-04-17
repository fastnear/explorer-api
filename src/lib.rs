pub mod api;
pub mod cache;
pub mod click;
#[cfg(feature = "openapi")]
pub mod openapi;
pub mod types;

use actix_web::{web, HttpResponse, Responder, Scope};

use crate::click::ClickDB;

#[derive(Clone)]
pub struct AppState {
    pub click_db: ClickDB,
}

pub fn api_v0_scope() -> Scope {
    web::scope("/v0")
        .service(api::v0::get_transactions)
        .service(api::v0::get_account)
        .service(api::v0::get_block)
        .service(api::v0::get_blocks)
        .service(api::v0::get_receipt)
}

pub async fn index() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../static/index.html"))
}

pub async fn skill_md() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/markdown; charset=utf-8")
        .body(include_str!("../static/skill.md"))
}
