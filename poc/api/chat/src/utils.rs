use std::collections::HashMap;

use anyhow::Result;
use spin_sdk::{
    http::{Request, Response},
    mysql::{self, Column, Decode, ParameterValue},
};

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

fn quick_response(s: http::StatusCode) -> Result<Response> {
    Ok(http::Response::builder().status(s).body(None)?)
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
        .get("Session-Id")
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

/// Checks if the current user owns the chat.
pub(crate) fn check_user_owns(db_url: &str, session: u64, model_id: u64) -> bool {
    let row_set = mysql::query(
        db_url,
        "SELECT id, created_by FROM chats WHERE id = ?",
        &vec![ParameterValue::Uint64(model_id)],
    );

    match row_set {
        Ok(row_set) => {
            let columns = get_column_lookup(&row_set.columns);

            match row_set.rows.first() {
                Some(row) => {
                    let created_by = u64::decode(&row[columns["created_by"]]);

                    match created_by {
                        Ok(created_by) => created_by == session,
                        Err(_) => false,
                    }
                }
                None => false,
            }
        }
        Err(_) => false,
    }
}
