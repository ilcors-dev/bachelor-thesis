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
pub(crate) struct CreateMessage {
    pub text: String,
}

impl CreateMessage {
    pub(crate) fn from_bytes(b: &Bytes) -> Result<Self> {
        let r: CreateMessage = serde_json::from_slice(b)?;
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
    pub(crate) fn from_bytes(b: &Bytes) -> Result<Self> {
        Ok(serde_json::from_slice(&b)?)
    }

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

    // pub(crate) fn insert(&self, db_url: &str) -> Result<()> {
    //     let binding = Ulid::new().to_string();
    //     let ulid = ParameterValue::Str(&binding.as_str());
    //     let text =
    //         as_param(&self.text).ok_or(anyhow!("The text field of the message is required"))?;

    //     let params = vec![ulid, text];

    //     mysql::execute(
    //         db_url,
    //         "INSERT INTO messages (ulid, text) VALUES (?, ?)",
    //         &params,
    //     )?;
    //     Ok(())
    // }

    // pub(crate) fn get_by_id(id: u64, db_url: &str) -> Result<Message> {
    //     let params = vec![ParameterValue::Uint64(id)];

    //     let row_set = mysql::query(
    //         db_url,
    //         "SELECT id, ulid, text, created_at, updated_at FROM messages WHERE id = ?",
    //         &params,
    //     )?;

    //     match row_set.rows.first() {
    //         Some(row) => Message::from_row(row),
    //         None => Err(anyhow!("Message not found for id '{:?}'", id)),
    //     }
    // }

    // pub(crate) fn update(&self, db_url: &str) -> Result<()> {
    //     match &self.id {
    //         Some(id) => {
    //             let params = vec![
    //                 ParameterValue::Str(&self.id),
    //                 as_nullable_param(&self.text),
    //                 ParameterValue::Str(id.as_str()),
    //             ];
    //             mysql::execute(
    //                 db_url,
    //                 "UPDATE messages SET id=?, text=? WHERE id=?",
    //                 &params,
    //             )?
    //         }
    //         None => {
    //             let params = vec![
    //                 as_nullable_param(&self.text),
    //                 ParameterValue::Str(self.id.as_str()),
    //             ];
    //             mysql::execute(db_url, "UPDATE messages SET text=? WHERE id=?", &params)?
    //         }
    //     }
    //     Ok(())
    // }

    // pub(crate) fn delete(&self, db_url: &str) -> Result<()> {
    //     match &self.id {
    //         Some(id) => {
    //             let params = vec![ParameterValue::Str(id.as_str())];
    //             mysql::execute(db_url, "DELETE FROM messages WHERE id=?", &params)?
    //         }
    //         None => {
    //             let params = vec![ParameterValue::Str(self.id.as_str())];
    //             mysql::execute(db_url, "DELETE FROM messages WHERE id=?", &params)?
    //         }
    //     }
    //     Ok(())
    // }
}
