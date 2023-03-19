mod config;
mod model;
mod utils;

use anyhow::Result;
use bytes::Bytes;
use config::Config;
use http::HeaderValue;
use model::{Chat, UpdateChat};
use spin_sdk::{
    http::{Request, Response},
    http_component,
    mysql::{self, ParameterValue},
};
use ulid::Ulid;
use utils::{
    bad_request, check_user_owns, get_column_lookup, get_session_id, method_not_allowed, not_found,
    ok, unauthorized,
};

use crate::model::CreateChat;

enum Api {
    Create(model::CreateChat),
    Show(u64),
    GetList,
    Update(model::UpdateChat),
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
        Api::Show(id) => handle_show(&cfg.db_url, id),
        Api::GetList => handle_get_list(&cfg.db_url),
        Api::Update(model) => handle_update(&cfg.db_url, session, model),
        Api::Delete(id) => handle_delete_by_id(&cfg.db_url, session, id),
        _ => not_found(),
    }
}

fn api_from_request(req: Request) -> Api {
    match *req.method() {
        http::Method::GET => match req.headers().get("spin-path-info") {
            None => Api::BadRequest,
            Some(v) => match get_id_from_route(v) {
                Ok(Some(id)) => Api::Show(id),
                Ok(None) => Api::GetList,
                Err(()) => Api::NotFound,
            },
        },
        http::Method::POST => {
            match CreateChat::from_bytes(req.body().as_ref().unwrap_or(&Bytes::new())) {
                Ok(model) => Api::Create(model),
                Err(_) => Api::BadRequest,
            }
        }
        http::Method::PUT => {
            match UpdateChat::from_bytes(req.body().as_ref().unwrap_or(&Bytes::new())) {
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

/// Gets a chat by id and returns it as JSON
fn handle_show(db_url: &str, chat_id: u64) -> Result<Response> {
    let params = vec![ParameterValue::Uint64(chat_id)];

    let row_set = mysql::query(
        db_url,
        "SELECT chats.id, chats.ulid, chats.name, chats.description, chats.created_at, chats.updated_at,
        s.session_id as creator_id, s.name as creator_name, s.emoji as creator_emoji
        FROM chats
        INNER JOIN sessions as s ON chats.created_by = s.id
        WHERE chats.id = ?",
        &params,
    )?;

    let columns = get_column_lookup(&row_set.columns);

    match row_set.rows.first() {
        Some(row) => {
            let model = Chat::from_row(&row, &columns)?;

            ok(serde_json::to_string(&model)?)
        }
        None => not_found(),
    }
}

fn handle_create(db_url: &str, session: u64, model: CreateChat) -> Result<Response> {
    let binding = Ulid::new().to_string();
    let description = match &model.description {
        Some(description) => ParameterValue::Str(description.as_str()),
        None => ParameterValue::DbNull,
    };

    let ulid = ParameterValue::Str(&binding.as_str());
    let name = ParameterValue::Str(&model.name.as_str());

    let created_by = ParameterValue::Uint64(session);

    let params = vec![ulid, name, description, created_by];

    mysql::execute(
        db_url,
        "INSERT INTO chats (ulid, name, description, created_by) VALUES (?, ?, ?, ?)",
        &params,
    )?;

    Ok(http::Response::builder()
        .status(http::StatusCode::CREATED)
        // .header(http::header::LOCATION, format!("/api/message/{}", model.id))
        .body(None)?)
}

fn handle_get_list(db_url: &str) -> Result<Response> {
    let params = vec![];

    let row_set = mysql::query(
        db_url,
        "SELECT chats.id, chats.ulid, chats.name, chats.description, chats.created_at, chats.updated_at,
        s.session_id as creator_id, s.name as creator_name, s.emoji as creator_emoji
        FROM chats
        INNER JOIN sessions as s ON chats.created_by = s.id",
        &params,
    )?;

    let columns = get_column_lookup(&row_set.columns);

    let mut models = vec![];

    for row in row_set.rows {
        let message = Chat::from_row(&row, &columns)?;
        models.push(message);
    }

    ok(serde_json::to_string(&models)?)
}

fn handle_update(db_url: &str, session: u64, model: UpdateChat) -> Result<Response> {
    if check_user_owns(db_url, session, model.id) == false {
        return unauthorized();
    }

    let id = ParameterValue::Uint64(model.id);

    let mut params = vec![];

    match &model.name {
        Some(name) => {
            let name = ParameterValue::Str(&name.as_str());

            params.push(name);
        }
        None => {}
    }

    match &model.description {
        Some(description) => {
            let description = ParameterValue::Str(&description.as_str());

            params.push(description);
        }
        None => {}
    }

    params.push(id);

    if params.len() == 1 {
        return Ok(http::Response::builder()
            .status(http::StatusCode::UNPROCESSABLE_ENTITY)
            // .header(http::header::LOCATION, format!("/api/message/{}", model.id))
            .body(None)?);
    }

    let sql = if params.len() == 3 {
        "UPDATE chats SET name = ?, description = ? WHERE id = ?"
    } else {
        "UPDATE chats SET name = ? WHERE id = ?"
    };

    mysql::execute(db_url, sql, &params)?;

    Ok(http::Response::builder()
        .status(http::StatusCode::OK)
        // .header(http::header::LOCATION, format!("/api/message/{}", model.id))
        .body(None)?)
}

fn handle_delete_by_id(db_url: &str, session: u64, id: u64) -> Result<Response> {
    if check_user_owns(db_url, session, id) == false {
        return unauthorized();
    }

    mysql::execute(
        db_url,
        "DELETE FROM chats WHERE id = ?",
        &vec![ParameterValue::Uint64(id)],
    )?;

    Ok(http::Response::builder()
        .status(http::StatusCode::OK)
        // .header(http::header::LOCATION, format!("/api/message/{}", model.id))
        .body(None)?)
}
