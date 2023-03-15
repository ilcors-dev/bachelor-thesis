use std::collections::HashMap;

use anyhow::{Ok, Result};
use bytes::Bytes;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use spin_sdk::mysql::Decode;

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
pub struct ChatCreator {
    pub session_id: String,
    pub name: String,
    pub emoji: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Chat {
    pub id: u64,
    pub ulid: String,
    pub created_by: ChatCreator,
    pub name: String,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Chat {
    pub fn from_row(row: &spin_sdk::mysql::Row, columns: &HashMap<&str, usize>) -> Result<Self> {
        let id = u64::decode(&row[columns["id"]])?;
        let ulid = String::decode(&row[columns["ulid"]])?;
        let name = String::decode(&row[columns["name"]])?;
        let description = Option::<String>::decode(&row[columns["description"]])?;

        let creator_id = String::decode(&row[columns["creator_id"]])?;
        let creator_name = String::decode(&row[columns["creator_name"]])?;
        let creator_emoji = String::decode(&row[columns["creator_emoji"]])?;

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
            created_by: ChatCreator {
                session_id: creator_id.to_string(),
                name: creator_name,
                emoji: creator_emoji,
            },
            name,
            description,
            created_at,
            updated_at,
        })
    }
}
