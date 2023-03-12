use spin_sdk::config::get;

const KEY_DB_URL: &str = "db_url";
const REDIS_ADDRESS_ENV: &str = "REDIS_ADDRESS";
const REDIS_CHANNEL_ENV: &str = "REDIS_CHANNEL";

#[derive(Debug)]
pub(crate) struct Config {
    pub db_url: String,
    pub redis_url: String,
    pub redis_channel: String,
}

impl Config {
    pub(crate) fn get() -> Config {
        Config {
            db_url: get(KEY_DB_URL).unwrap(),
            redis_url: std::env::var(REDIS_ADDRESS_ENV).unwrap(),
            redis_channel: std::env::var(REDIS_CHANNEL_ENV).unwrap(),
        }
    }
}
