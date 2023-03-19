use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Session {
    pub session_id: u64,
    pub name: String,
    pub emoji: String,
    pub last_active: NaiveDateTime,
}
