use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub id: i128,
    pub ulid: String,
    pub text: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InsertMessage {
    pub text: String,
}

impl Message {
    pub fn new(
        id: i128,
        ulid: String,
        text: String,
        created_at: chrono::NaiveDateTime,
        updated_at: chrono::NaiveDateTime,
    ) -> Message {
        Message {
            id,
            ulid,
            text,
            created_at,
            updated_at,
        }
    }
}
