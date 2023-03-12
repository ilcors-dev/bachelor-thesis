mod config;
mod model;
mod utils;

use std::collections::HashMap;

use anyhow::{anyhow, Result};
use config::Config;
use model::Session;
use spin_sdk::{
    http::{Request, Response},
    http_component, redis,
};
use utils::{bad_request, get_session_id, method_not_allowed, not_found, unauthorized};

enum Api {
    Create,
    Get,
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
        Api::Create => handle_create(&cfg.redis_url, session),
        Api::Get => handle_get(&cfg.redis_url),
        _ => not_found(),
    }
}

fn api_from_request(req: Request) -> Api {
    match *req.method() {
        http::Method::GET => Api::Get,
        http::Method::POST => Api::Create,
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

    let mut store: HashMap<u64, Session> = match cached {
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

fn handle_create(redis_url: &str, session_id: u64) -> Result<Response> {
    let mut store = get_store(redis_url)?;

    let session = Session {
        session_id,
        last_active: chrono::Local::now().naive_utc(),
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

fn handle_get(redis_url: &str) -> Result<Response> {
    Ok(http::Response::builder()
        .status(http::StatusCode::OK)
        .body(Some(serde_json::to_string(&get_store(redis_url)?)?.into()))?)
}
