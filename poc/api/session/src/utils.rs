use std::collections::HashMap;

use anyhow::Result;
use spin_sdk::{http::Response, mysql::Column};

pub(crate) fn created(payload: String) -> Result<Response> {
    Ok(http::Response::builder()
        .status(http::StatusCode::CREATED)
        .header(http::header::CONTENT_TYPE, "application/json")
        .body(Some(payload.into()))?)
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
