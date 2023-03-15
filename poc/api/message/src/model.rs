use std::collections::HashMap;

use anyhow::{Ok, Result};
use bytes::Bytes;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use spin_sdk::mysql::Decode;

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct CreateMessage {
    pub chat_id: u64,
    pub text: String,
}

impl CreateMessage {
    pub(crate) fn from_bytes(b: &Bytes) -> Result<Self> {
        let r: CreateMessage = serde_json::from_slice(b)?;
        Ok(r)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct UpdateMessage {
    pub id: u64,
    pub text: String,
}

impl UpdateMessage {
    pub(crate) fn from_bytes(b: &Bytes) -> Result<Self> {
        let r: UpdateMessage = serde_json::from_slice(b)?;
        Ok(r)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Message {
    pub id: u64,
    pub ulid: String,
    pub text: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Message {
    pub fn from_row(row: &spin_sdk::mysql::Row, columns: &HashMap<&str, usize>) -> Result<Self> {
        let id = u64::decode(&row[columns["id"]])?;
        let ulid = String::decode(&row[columns["ulid"]])?;
        let text = String::decode(&row[columns["text"]])?;

        let created_at = NaiveDateTime::parse_from_str(
            &String::decode(&row[columns["created_at"]])?,
            "%Y-%m-%d %H:%M:%S",
        )?;

        let updated_at = NaiveDateTime::parse_from_str(
            &String::decode(&row[columns["updated_at"]])?,
            "%Y-%m-%d %H:%M:%S",
        )?;

        Ok(Message {
            id,
            ulid,
            text,
            created_at,
            updated_at,
        })
    }
}
