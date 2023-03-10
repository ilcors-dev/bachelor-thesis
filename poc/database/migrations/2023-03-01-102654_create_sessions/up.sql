-- Your SQL goes here
CREATE TABLE
    sessions (
        id BIGINT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
        session_id VARCHAR(255) NOT NULL,
        payload TEXT, -- a JSON string
        -- a date string, note that this is not the appropriate data type for a date, but the MySQL driver for WASI & Rust doesn't support the date type
        expires_at VARCHAR(255) NOT NULL,
        created_at VARCHAR(255) NOT NULL -- a date string
    );

-- this is needed for the MySQL driver for WASI & Rust to work as it doesn't support the date type, kinda hacky..
CREATE TRIGGER sessions_before_insert BEFORE INSERT ON sessions FOR EACH ROW BEGIN
SET
    NEW.expires_at = NOW() + INTERVAL 1 DAY,
    NEW.created_at = NOW();

END;