mod config;
mod model;
mod utils;

use anyhow::{anyhow, Result};
use bytes::Bytes;
use config::Config;
use http::HeaderValue;
use model::Message;
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

use crate::model::CreateMessage;

enum Api {
    Create(model::CreateMessage),
    ReadById(u64),
    ReadAll,
    Update(model::Message),
    Delete(model::Message),
    BadRequest,
    NotFound,
    MethodNotAllowed,
}

fn get_id_from_route(header_value: &HeaderValue) -> Result<Option<u64>, ()> {
    match header_value.to_str() {
        Ok(value) => {
            let segment = value.split('/').last();

            match segment {
                Some("") => Ok(None),
                Some(id_as_str) => match id_as_str.parse::<u64>() {
                    Ok(id) => Ok(Some(id)),
                    Err(_) => Err(()),
                },
                _ => Err(()),
            }
        }
        Err(_) => Err(()),
    }
}

#[http_component]
fn message_api(req: Request) -> Result<Response> {
    let cfg = Config::get();

    match api_from_request(req) {
        Api::BadRequest => bad_request(),
        Api::MethodNotAllowed => method_not_allowed(),
        Api::Create(model) => handle_create(&cfg.db_url, model),
        // Api::Update(model) => handle_update(&cfg.db_url, model),
        Api::ReadById(id) => handle_read_by_id(&cfg.db_url, id),
        Api::ReadAll => handle_read_all(&cfg.db_url),
        // Api::Delete(id) => handle_delete_by_handle(&cfg.db_url, id),
        _ => not_found(),
    }
}

fn api_from_request(req: Request) -> Api {
    println!("req.headers(): {:?}", req.headers());

    match *req.method() {
        http::Method::POST => {
            println!("req.body(): {:?}", req.body());
            match CreateMessage::from_bytes(req.body().as_ref().unwrap_or(&Bytes::new())) {
                Ok(model) => Api::Create(model),
                Err(_) => Api::BadRequest,
            }
        }
        http::Method::GET => match req.headers().get("spin-path-info") {
            None => Api::BadRequest,
            Some(v) => match get_id_from_route(v) {
                Ok(Some(id)) => Api::ReadById(id),
                Ok(None) => Api::ReadAll,
                Err(()) => Api::NotFound,
            },
        },
        _ => Api::MethodNotAllowed,
    }
}

fn handle_create(db_url: &str, model: CreateMessage) -> Result<Response> {
    let binding = Ulid::new().to_string();
    let ulid = ParameterValue::Str(&binding.as_str());
    let text = ParameterValue::Str(&model.text.as_str());

    let params = vec![ulid, text];

    mysql::execute(
        db_url,
        "INSERT INTO messages (ulid, text) VALUES (?, ?)",
        &params,
    )?;

    Ok(http::Response::builder()
        .status(http::StatusCode::CREATED)
        // .header(http::header::LOCATION, format!("/api/message/{}", model.id))
        .body(None)?)
}

fn handle_read_by_id(db_url: &str, id: u64) -> Result<Response> {
    let params = vec![ParameterValue::Uint64(id)];

    let row_set = mysql::query(
        db_url,
        "SELECT id, ulid, text, created_at, updated_at FROM messages WHERE id = ?",
        &params,
    )?;

    let columns = get_column_lookup(&row_set.columns);

    println!("row_set: {:?}", row_set.rows);

    match row_set.rows.first() {
        Some(row) => {
            let model = Message::from_row(row, &columns)?;
            ok(serde_json::to_string(&model)?)
        }
        None => not_found(),
    }
}

fn handle_read_all(db_url: &str) -> Result<Response> {
    let params = vec![];

    let row_set = mysql::query(
        db_url,
        "SELECT id, ulid, text, created_at, updated_at FROM messages",
        &params,
    )?;

    let columns = get_column_lookup(&row_set.columns);

    let mut models = vec![];

    for row in row_set.rows {
        let message = Message::from_row(&row, &columns)?;
        models.push(message);
    }

    ok(serde_json::to_string(&models)?)
}

// fn handle_update(db_url: &str, model: Message) -> Result<Response> {
//     model.update(&db_url)?;
//     handle_read_by_handle(&db_url, model.id)
// }

// fn handle_delete_by_handle(db_url: &str, model: Message) -> Result<Response> {
//     match model.delete(&db_url) {
//         Ok(_) => no_content(),
//         Err(_) => internal_server_error(String::from("Error while deleting Message")),
//     }
// }
