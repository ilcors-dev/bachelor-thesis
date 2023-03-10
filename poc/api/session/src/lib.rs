mod config;
mod model;
mod utils;

use anyhow::{anyhow, Result};
use bytes::Bytes;
use config::Config;
use http::HeaderValue;
use model::Session;
use spin_sdk::{
    http::{Request, Response},
    http_component,
    mysql::{self, ParameterValue},
};
use ulid::Ulid;
use utils::{
    bad_request, get_column_lookup, internal_server_error, method_not_allowed, no_content,
    not_found, ok,
};

enum Api {
    Create,
    BadRequest,
    NotFound,
    MethodNotAllowed,
}

#[http_component]
fn message_service(req: Request) -> Result<Response> {
    let cfg = Config::get();

    match api_from_request(req) {
        Api::BadRequest => bad_request(),
        Api::MethodNotAllowed => method_not_allowed(),
        Api::Create => handle_create(&cfg.db_url),
        _ => not_found(),
    }
}

fn api_from_request(req: Request) -> Api {
    match *req.method() {
        http::Method::GET => Api::Create,
        _ => Api::MethodNotAllowed,
    }
}

fn handle_create(db_url: &str) -> Result<Response> {
    let session_id = Ulid::new().to_string();

    let params = vec![ParameterValue::Str(&session_id.as_str())];

    mysql::execute(
        db_url,
        "INSERT INTO sessions (session_id) VALUES (?)",
        &params,
    )?;

    Ok(http::Response::builder()
        .status(http::StatusCode::CREATED)
        .header("session_id", session_id)
        // .header(http::header::LOCATION, format!("/api/message/{}", model.id))
        .body(None)?)
}
