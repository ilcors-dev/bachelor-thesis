use std::collections::HashMap;

use anyhow::{Ok, Result};
use bytes::Bytes;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use spin_sdk::mysql::{Decode, ParameterValue};

fn as_param<'a>(value: &'a Option<String>) -> Option<ParameterValue<'a>> {
    match value {
        Some(value) => Some(ParameterValue::Str(value.as_str())),
        None => None,
    }
}

fn as_nullable_param<'a>(value: &'a Option<String>) -> ParameterValue<'a> {
    match as_param(value) {
        Some(value) => value,
        None => ParameterValue::DbNull,
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Session {
    pub id: u64,
    pub session_id: String,
    pub payload: String,
    pub expires_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

impl Session {
    pub(crate) fn from_bytes(b: &Bytes) -> Result<Self> {
        Ok(serde_json::from_slice(&b)?)
    }

    pub fn from_row(row: &spin_sdk::mysql::Row, columns: &HashMap<&str, usize>) -> Result<Self> {
        let id = u64::decode(&row[columns["id"]])?;
        let session_id = String::decode(&row[columns["session_id"]])?;
        let payload = String::decode(&row[columns["payload"]])?;

        let expires_at = NaiveDateTime::parse_from_str(
            &String::decode(&row[columns["expires_at"]])?,
            "%Y-%m-%d %H:%M:%S",
        )?;

        let created_at = NaiveDateTime::parse_from_str(
            &String::decode(&row[columns["created_at"]])?,
            "%Y-%m-%d %H:%M:%S",
        )?;

        Ok(Session {
            id,
            session_id,
            payload,
            expires_at,
            created_at,
        })
    }
}
