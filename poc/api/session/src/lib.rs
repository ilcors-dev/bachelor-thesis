mod config;
mod model;
mod utils;

use anyhow::Result;
use config::Config;
use model::SelectSession;
use spin_sdk::{
    http::{Request, Response},
    http_component,
    mysql::{self, ParameterValue},
};
use ulid::Ulid;
use utils::{created, get_column_lookup, not_found};

enum Api {
    Create,
    NotFound,
}

#[http_component]
fn message_service(req: Request) -> Result<Response> {
    let cfg = Config::get();

    match api_from_request(req) {
        Api::Create => handle_create(&cfg.db_url),
        _ => not_found(),
    }
}

fn api_from_request(req: Request) -> Api {
    match *req.method() {
        http::Method::GET => Api::Create,
        _ => Api::NotFound,
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

    let row_set = mysql::query(
        db_url,
        "SELECT session_id, expires_at, created_at FROM sessions WHERE session_id = ? LIMIT 1",
        &params,
    )?;

    let columns = get_column_lookup(&row_set.columns);

    match row_set.rows.first() {
        Some(row) => {
            let model = SelectSession::from_row(row, &columns)?;

            created(serde_json::to_string(&model)?)
        }
        None => not_found(),
    }
}
