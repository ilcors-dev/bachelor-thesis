use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Session {
    pub session_id: u64,
    pub last_active: NaiveDateTime,
}
