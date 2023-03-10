mod config;
mod model;
mod utils;

use anyhow::{anyhow, Result};
use bytes::Bytes;
use config::Config;
use http::HeaderValue;
use model::{Message, UpdateMessage};
use spin_sdk::{
    http::{Request, Response},
    http_component,
    mysql::{self, ParameterValue},
};
use ulid::Ulid;
use utils::{
    bad_request, check_user_owns, get_column_lookup, get_session_id, internal_server_error,
    method_not_allowed, no_content, not_found, ok, unauthorized,
};

use crate::model::CreateMessage;

enum Api {
    Create(model::CreateMessage),
    GetLatestFromChat(u64, Option<u64>),
    Update(model::UpdateMessage),
    Delete(u64),
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

fn get_query_param(query: &str, name: &str) -> Result<Option<String>, ()> {
    let params = query.split('&');

    for param in params {
        let mut parts = param.split('=');

        let key = parts.next();
        let value = parts.next();

        match (key, value) {
            (Some(key), Some(value)) => {
                if key == name {
                    return Ok(Some(value.to_string()));
                }
            }
            _ => {}
        }
    }

    Ok(None)
}

#[http_component]
fn message_service(req: Request) -> Result<Response> {
    let cfg = Config::get();

    let session = get_session_id(&cfg.db_url, &req).unwrap_or_else(|| 0);

    if session == 0 {
        return unauthorized();
    }

    match api_from_request(req) {
        Api::BadRequest => bad_request(),
        Api::MethodNotAllowed => method_not_allowed(),
        Api::Create(model) => handle_create(&cfg.db_url, session, model),
        Api::GetLatestFromChat(chat_id, fetch_from_message_id) => {
            handle_get_latest_from_chat(&cfg.db_url, chat_id, fetch_from_message_id)
        }
        Api::Update(model) => handle_update(&cfg.db_url, session, model),
        Api::Delete(id) => handle_delete_by_id(&cfg.db_url, session, id),
        _ => not_found(),
    }
}

fn api_from_request(req: Request) -> Api {
    match *req.method() {
        http::Method::POST => {
            match CreateMessage::from_bytes(req.body().as_ref().unwrap_or(&Bytes::new())) {
                Ok(model) => Api::Create(model),
                Err(_) => Api::BadRequest,
            }
        }
        http::Method::GET => match req.uri().query() {
            Some(query) => {
                let chat_id = match get_query_param(query, "chat_id") {
                    Ok(Some(id)) => match id.parse::<u64>() {
                        Ok(id) => id,
                        Err(_) => return Api::BadRequest,
                    },
                    Ok(None) => return Api::BadRequest,
                    Err(_) => return Api::BadRequest,
                };

                let fetch_from_message_id = get_query_param(query, "fetch_from_message_id")
                    .unwrap_or_else(|_| None)
                    .and_then(|id| id.parse::<u64>().ok());

                Api::GetLatestFromChat(chat_id, fetch_from_message_id)
            }
            None => Api::BadRequest,
        },
        http::Method::PUT => {
            match UpdateMessage::from_bytes(req.body().as_ref().unwrap_or(&Bytes::new())) {
                Ok(model) => Api::Update(model),
                Err(_) => Api::BadRequest,
            }
        }
        http::Method::DELETE => match req.headers().get("spin-path-info") {
            None => Api::BadRequest,
            Some(v) => match get_id_from_route(v) {
                Ok(Some(id)) => Api::Delete(id),
                Ok(None) => Api::NotFound,
                Err(()) => Api::NotFound,
            },
        },
        _ => Api::MethodNotAllowed,
    }
}

fn handle_create(db_url: &str, session: u64, model: CreateMessage) -> Result<Response> {
    let chat_id = ParameterValue::Uint64(model.chat_id);
    let sender_id = ParameterValue::Uint64(session);
    let binding = Ulid::new().to_string();
    let ulid = ParameterValue::Str(&binding.as_str());
    let text = ParameterValue::Str(&model.text.as_str());

    let params = vec![chat_id, sender_id, ulid, text];

    mysql::execute(
        db_url,
        "INSERT INTO messages (chat_id, sender_id, ulid, text) VALUES (?, ?, ?, ?)",
        &params,
    )?;

    Ok(http::Response::builder()
        .status(http::StatusCode::CREATED)
        // .header(http::header::LOCATION, format!("/api/message/{}", model.id))
        .body(None)?)
}

// fn handle_read_by_id(db_url: &str, id: u64) -> Result<Response> {
//     let params = vec![ParameterValue::Uint64(id)];

//     let row_set = mysql::query(
//         db_url,
//         "SELECT id, ulid, text, created_at, updated_at FROM messages WHERE id = ?",
//         &params,
//     )?;

//     let columns = get_column_lookup(&row_set.columns);

//     match row_set.rows.first() {
//         Some(row) => {
//             let model = Message::from_row(row, &columns)?;
//             ok(serde_json::to_string(&model)?)
//         }
//         None => not_found(),
//     }
// }

fn handle_get_latest_from_chat(
    db_url: &str,
    chat_id: u64,
    fetch_from_message_id: Option<u64>,
) -> Result<Response> {
    let row_set = match fetch_from_message_id {
        Some(id) => mysql::query(
            db_url,
            "SELECT id, ulid, text, created_at, updated_at FROM messages WHERE chat_id = ? AND id > ?",
            &vec![ParameterValue::Uint64(chat_id), ParameterValue::Uint64(id)],
        )?,
        None => mysql::query(
            db_url,
            "SELECT id, ulid, text, created_at, updated_at FROM messages WHERE chat_id = ?",
            &vec![ParameterValue::Uint64(chat_id)],
        )?,
    };

    let columns = get_column_lookup(&row_set.columns);

    let mut models = vec![];

    for row in row_set.rows {
        let message = Message::from_row(&row, &columns)?;
        models.push(message);
    }

    ok(serde_json::to_string(&models)?)
}

fn handle_update(db_url: &str, session: u64, model: UpdateMessage) -> Result<Response> {
    if check_user_owns(db_url, session, model.id) == false {
        return unauthorized();
    }

    let id = ParameterValue::Uint64(model.id);
    let text = ParameterValue::Str(&model.text);

    let params = vec![text, id];

    mysql::execute(db_url, "UPDATE messages SET text = ? WHERE id = ?", &params)?;

    Ok(http::Response::builder()
        .status(http::StatusCode::OK)
        // .header(http::header::LOCATION, format!("/api/message/{}", model.id))
        .body(None)?)
}

fn handle_delete_by_id(db_url: &str, session: u64, id: u64) -> Result<Response> {
    if check_user_owns(db_url, session, id) == false {
        return unauthorized();
    }

    let params = vec![ParameterValue::Uint64(id)];

    mysql::execute(db_url, "DELETE FROM messages WHERE id = ?", &params)?;

    Ok(http::Response::builder()
        .status(http::StatusCode::OK)
        // .header(http::header::LOCATION, format!("/api/message/{}", model.id))
        .body(None)?)
}
