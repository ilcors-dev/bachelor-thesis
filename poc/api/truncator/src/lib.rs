mod config;
mod utils;

use anyhow::Result;
use config::Config;
use spin_sdk::{
    http::{Request, Response},
    http_component,
    mysql::{self, Decode, ParameterValue},
};
use utils::{bad_request, method_not_allowed, not_found};

enum Api {
    Truncate,
    BadRequest,
    MethodNotAllowed,
}

#[http_component]
fn truncator_service(req: Request) -> Result<Response> {
    let cfg = Config::get();

    match api_from_request(req) {
        Api::BadRequest => bad_request(),
        Api::MethodNotAllowed => method_not_allowed(),
        Api::Truncate => handle_truncate(&cfg.db_url, &cfg.db_name),
        _ => not_found(),
    }
}

fn api_from_request(req: Request) -> Api {
    match *req.method() {
        http::Method::GET => Api::Truncate,
        _ => Api::MethodNotAllowed,
    }
}

fn handle_truncate(db_url: &str, db_name: &str) -> Result<Response> {
    mysql::execute(db_url, "SET FOREIGN_KEY_CHECKS=0;", &[])?;

    let messages = format!("TRUNCATE TABLE messages;");
    let chats = format!("TRUNCATE TABLE chats;");
    let sessions = format!("TRUNCATE TABLE sessions;");

    mysql::execute(db_url, &messages, &[])?;
    mysql::execute(db_url, &chats, &[])?;
    mysql::execute(db_url, &sessions, &[])?;

    mysql::execute(db_url, "SET FOREIGN_KEY_CHECKS=1;", &[])?;

    println!("Truncated tables in database: {}", db_name);

    Ok(http::Response::builder()
        .status(http::StatusCode::OK)
        // .header(http::header::LOCATION, format!("/api/message/{}", model.id))
        .body(None)?)
}
