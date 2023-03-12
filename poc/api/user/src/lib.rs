mod config;
mod model;
mod utils;

use anyhow::{anyhow, Result};
use bytes::Bytes;
use config::Config;
use http::HeaderValue;
use model::{Session, Store};
use spin_sdk::{
    http::{Request, Response},
    http_component, redis,
};
use utils::{
    bad_request, check_user_owns, get_column_lookup, get_session_id, internal_server_error,
    method_not_allowed, not_found, ok, unauthorized,
};

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

fn handle_create(redis_url: &str, session_id: u64) -> Result<Response> {
    let store = redis::get(&redis_url, "sessions");

    let mut store: Store = match store {
        Ok(store) => {
            let json = String::from_utf8(store).map_err(|_| anyhow!("Deserialize Error"))?;

            serde_json::from_str(&json).map_err(|_| anyhow!("Json conversion error"))?
        }
        Err(_) => Store { sessions: vec![] },
    };

    store.sessions.push(Session {
        session_id,
        last_active: chrono::Utc::now().naive_utc(),
    });

    println!("store: {:?}", store);

    let json = serde_json::to_string(&store).map_err(|_| anyhow!("Serialize Error"))?;

    println!("json: {}", json);
    redis::set(&redis_url, "sessions", &b"test"[..]).map_err(|_| anyhow!("Redis Error"))?;

    Ok(http::Response::builder()
        .status(http::StatusCode::OK)
        .body(Some(serde_json::to_string(&store)?.into()))?)
}

fn handle_get(db_url: &str) -> Result<Response> {
    // get the sessions from redis stored in a json containing multiple Session structs
    // store the session_id in redis in a Session struct with the current time

    let store = redis::get(&db_url, "sessions").map_err(|_| anyhow!("Redis Error"))?;

    let json = String::from_utf8(store).map_err(|_| anyhow!("Deserialize Error"))?;

    let store: Store = serde_json::from_str(&json).map_err(|_| anyhow!("Redis Error"))?;

    Ok(http::Response::builder()
        .status(http::StatusCode::OK)
        .body(Some(serde_json::to_string(&store)?.into()))?)
}
