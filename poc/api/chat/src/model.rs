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
pub(crate) struct CreateChat {
    pub name: String,
    pub description: Option<String>,
}

impl CreateChat {
    pub(crate) fn from_bytes(b: &Bytes) -> Result<Self> {
        let r: CreateChat = serde_json::from_slice(b)?;
        Ok(r)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct UpdateChat {
    pub id: u64,
    pub name: Option<String>,
    pub description: Option<String>,
}

impl UpdateChat {
    pub(crate) fn from_bytes(b: &Bytes) -> Result<Self> {
        let r: UpdateChat = serde_json::from_slice(b)?;
        Ok(r)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Chat {
    pub id: u64,
    pub ulid: String,
    pub created_by: Option<u64>,
    pub name: String,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Chat {
    pub(crate) fn from_bytes(b: &Bytes) -> Result<Self> {
        Ok(serde_json::from_slice(&b)?)
    }

    pub fn from_row(row: &spin_sdk::mysql::Row, columns: &HashMap<&str, usize>) -> Result<Self> {
        let id = u64::decode(&row[columns["id"]])?;
        let ulid = String::decode(&row[columns["ulid"]])?;
        let created_by = Option::<u64>::decode(&row[columns["created_by"]])?;
        let name = String::decode(&row[columns["name"]])?;
        let description = Option::<String>::decode(&row[columns["description"]])?;

        let created_at = NaiveDateTime::parse_from_str(
            &String::decode(&row[columns["created_at"]])?,
            "%Y-%m-%d %H:%M:%S",
        )?;

        let updated_at = NaiveDateTime::parse_from_str(
            &String::decode(&row[columns["updated_at"]])?,
            "%Y-%m-%d %H:%M:%S",
        )?;

        Ok(Chat {
            id,
            ulid,
            created_by,
            name,
            description,
            created_at,
            updated_at,
        })
    }
}
