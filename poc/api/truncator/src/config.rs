use spin_sdk::config::get;

const KEY_DB_URL: &str = "db_url";
const KEY_DB_NAME: &str = "db_name";

#[derive(Debug)]
pub(crate) struct Config {
    pub db_url: String,
    pub db_name: String,
}

impl Config {
    pub(crate) fn get() -> Config {
        Config {
            db_url: get(KEY_DB_URL).unwrap(),
            db_name: get(KEY_DB_NAME).unwrap(),
        }
    }
}
