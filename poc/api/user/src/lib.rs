mod config;
mod model;
mod utils;

use std::collections::{hash_map::Entry, HashMap};

use anyhow::{anyhow, Result};
use config::Config;
use model::Session;
use spin_sdk::{
    http::{internal_server_error, Request, Response},
    http_component,
    mysql::{self, Decode, ParameterValue},
    redis,
};
use utils::{
    bad_request, get_column_lookup, get_session_id, method_not_allowed, not_found, unauthorized,
};

enum Api {
    Create,
    Get,
    Put,
    BadRequest,
    NotFound,
    MethodNotAllowed,
}

#[http_component]
fn user_service(req: Request) -> Result<Response> {
    let cfg = Config::get();

    let session = get_session_id(&cfg.db_url, &req).unwrap_or_else(|| 0);

    if session == 0 {
        return unauthorized();
    }

    match api_from_request(req) {
        Api::BadRequest => bad_request(),
        Api::MethodNotAllowed => method_not_allowed(),
        Api::Create => handle_create(&cfg.db_url, &cfg.redis_url, session),
        Api::Get => handle_get(&cfg.redis_url),
        Api::Put => handle_put(&cfg.redis_url, session),
        _ => not_found(),
    }
}

fn api_from_request(req: Request) -> Api {
    match *req.method() {
        http::Method::GET => Api::Get,
        http::Method::POST => Api::Create,
        http::Method::PUT => Api::Put,
        _ => Api::MethodNotAllowed,
    }
}

/// Removes expired sessions from the store
///
/// Sessions are considered expired if they have not been active for x minutes
///
/// The store is modified by reference!
fn remove_expired_sessions(store: &mut HashMap<u64, Session>) {
    let now = chrono::Local::now().naive_utc();

    let mut expired = Vec::new();

    for (id, session) in store.iter() {
        if now - session.last_active > chrono::Duration::minutes(2) {
            expired.push(*id);
        }
    }

    for id in expired {
        store.remove(&id);
    }
}

/// Get the sessions from redis stored in a json containing multiple Session structs
///
/// If error, return an empty HashMap
fn get_store(redis_url: &str) -> Result<HashMap<u64, Session>> {
    let cached = redis::get(&redis_url, "sessions");

    let store: HashMap<u64, Session> = match cached {
        Ok(store) => {
            if store.len() == 0 {
                HashMap::new()
            } else {
                let json = String::from_utf8(store).map_err(|_| anyhow!("Deserialize Error"))?;

                let store: HashMap<u64, Session> =
                    serde_json::from_str(&json).map_err(|_| anyhow!("Redis Error"))?;

                store
            }
        }
        Err(_) => HashMap::new(),
    };

    Ok(store)
}

/// Registers a new user online status in redis
fn handle_create(db_url: &str, redis_url: &str, session_id: u64) -> Result<Response> {
    let mut store = get_store(redis_url)?;

    let row_set = mysql::query(
        &db_url,
        "SELECT name, emoji FROM sessions WHERE id = ?",
        &vec![ParameterValue::Uint64(session_id)],
    )?;

    let columns = get_column_lookup(&row_set.columns);

    let row = row_set.rows.first();

    let session = match row {
        Some(row) => {
            let name = String::decode(&row[columns["name"]])?;
            let emoji = String::decode(&row[columns["emoji"]])?;

            Session {
                session_id,
                name,
                emoji,
                last_active: chrono::Local::now().naive_utc(),
            }
        }
        None => return internal_server_error(),
    };

    store.insert(session_id, session);

    // remove expired sessions
    remove_expired_sessions(&mut store);

    let json = serde_json::to_string(&store).map_err(|_| anyhow!("Serialize Error"))?;

    redis::set(&redis_url, "sessions", json.as_bytes()).map_err(|_| anyhow!("Redis Error"))?;

    Ok(http::Response::builder()
        .status(http::StatusCode::OK)
        .body(Some(serde_json::to_string(&store)?.into()))?)
}

/// Returns the sessions stored in redis as a json
fn handle_get(redis_url: &str) -> Result<Response> {
    Ok(http::Response::builder()
        .status(http::StatusCode::OK)
        .body(Some(serde_json::to_string(&get_store(redis_url)?)?.into()))?)
}

/// Updates the last_active timestamp of the session
///
/// If the session does not exist, return an internal server error
fn handle_put(redis_url: &str, session_id: u64) -> Result<Response> {
    let mut store = get_store(redis_url)?;

    match store
        .entry(session_id)
        .and_modify(|session| session.last_active = chrono::Local::now().naive_utc())
    {
        Entry::Occupied(_) => {}
        Entry::Vacant(_) => return not_found(),
    };

    // remove expired sessions
    remove_expired_sessions(&mut store);

    let json = serde_json::to_string(&store).map_err(|_| anyhow!("Serialize Error"))?;

    redis::set(&redis_url, "sessions", json.as_bytes()).map_err(|_| anyhow!("Redis Error"))?;

    Ok(http::Response::builder()
        .status(http::StatusCode::OK)
        .body(Some(serde_json::to_string(&store)?.into()))?)
}
