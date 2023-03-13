use std::collections::HashMap;

use anyhow::{Ok, Result};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use spin_sdk::mysql::{Decode, ParameterValue};

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct SelectSession {
    pub session_id: String,
    pub name: String,
    pub emoji: String,
    pub expires_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

impl SelectSession {
    pub fn from_row(row: &spin_sdk::mysql::Row, columns: &HashMap<&str, usize>) -> Result<Self> {
        let session_id = String::decode(&row[columns["session_id"]])?;
        let name = String::decode(&row[columns["name"]])?;
        let emoji = String::decode(&row[columns["emoji"]])?;

        let expires_at = NaiveDateTime::parse_from_str(
            &String::decode(&row[columns["expires_at"]])?,
            "%Y-%m-%d %H:%M:%S",
        )?;

        let created_at = NaiveDateTime::parse_from_str(
            &String::decode(&row[columns["created_at"]])?,
            "%Y-%m-%d %H:%M:%S",
        )?;

        Ok(SelectSession {
            session_id,
            name,
            emoji,
            expires_at,
            created_at,
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Session {
    pub id: u64,
    pub session_id: String,
    pub name: Option<String>,
    pub emoji: Option<String>,
    pub payload: Option<String>,
    pub expires_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

impl Session {
    pub fn from_row(row: &spin_sdk::mysql::Row, columns: &HashMap<&str, usize>) -> Result<Self> {
        let id = u64::decode(&row[columns["id"]])?;
        let session_id = String::decode(&row[columns["session_id"]])?;
        let name = String::decode(&row[columns["name"]]).ok();
        let emoji = String::decode(&row[columns["emoji"]]).ok();

        let payload = String::decode(&row[columns["payload"]]).ok();

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
            name,
            emoji,
            payload,
            expires_at,
            created_at,
        })
    }
}
