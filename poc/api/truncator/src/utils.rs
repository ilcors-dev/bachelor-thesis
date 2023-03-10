use std::collections::HashMap;

use anyhow::Result;
use spin_sdk::{
    http::{Request, Response},
    mysql::{self, Column, Decode, ParameterValue},
};

pub(crate) fn internal_server_error(err: String) -> Result<Response> {
    Ok(http::Response::builder()
        .status(http::StatusCode::INTERNAL_SERVER_ERROR)
        .header(http::header::CONTENT_TYPE, "text/plain")
        .body(Some(err.into()))?)
}

pub(crate) fn ok(payload: String) -> Result<Response> {
    Ok(http::Response::builder()
        .status(http::StatusCode::OK)
        .header(http::header::CONTENT_TYPE, "application/json")
        .body(Some(payload.into()))?)
}

pub(crate) fn method_not_allowed() -> Result<Response> {
    quick_response(http::StatusCode::METHOD_NOT_ALLOWED)
}

pub(crate) fn bad_request() -> Result<Response> {
    quick_response(http::StatusCode::BAD_REQUEST)
}

pub(crate) fn unauthorized() -> Result<Response> {
    quick_response(http::StatusCode::UNAUTHORIZED)
}

pub(crate) fn not_found() -> Result<Response> {
    quick_response(http::StatusCode::NOT_FOUND)
}

pub(crate) fn no_content() -> Result<Response> {
    quick_response(http::StatusCode::NO_CONTENT)
}

fn quick_response(s: http::StatusCode) -> Result<Response> {
    Ok(http::Response::builder().status(s).body(None)?)
}

pub(crate) fn get_params_from_route(route: &str) -> Vec<String> {
    route
        .split('/')
        .flat_map(|s| if s == "" { None } else { Some(s.to_string()) })
        .collect::<Vec<String>>()
}

pub(crate) fn get_last_param_from_route(route: &str) -> Option<String> {
    get_params_from_route(route).last().cloned()
}

pub(crate) fn get_column_lookup<'a>(columns: &'a Vec<Column>) -> HashMap<&'a str, usize> {
    columns
        .iter()
        .enumerate()
        .map(|(i, c)| (c.name.as_str(), i))
        .collect::<HashMap<&str, usize>>()
}

/// Get the session id from the request header.
pub(crate) fn get_session_id(db_url: &str, req: &Request) -> Option<u64> {
    req.headers()
        .get("session_id")
        .and_then(|header_value| header_value.to_str().ok())
        .map(|value| {
            let params = vec![ParameterValue::Str(value)];

            let query = "SELECT id FROM sessions WHERE session_id = ?";

            let result = mysql::query(db_url, query, &params);

            match result {
                Ok(rows) => {
                    let column_lookup = get_column_lookup(&rows.columns);

                    rows.rows
                        .first()
                        .and_then(|row| u64::decode(&row[column_lookup["id"]]).ok())
                }
                Err(_) => None,
            }
        })
        .flatten()
}
