-- Your SQL goes here
CREATE TABLE
    sessions (
        id BIGINT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
        session_id VARCHAR(255) NOT NULL,
        payload TEXT NOT NULL,
        expires_at TIMESTAMP NOT NULL,
        created_at TIMESTAMP NOT NULL,
    );

CREATE TRIGGER sessions_before_insert BEFORE INSERT ON sessions FOR EACH ROW BEGIN
SET
    NEW.expires_at = NOW () + INTERVAL 1 DAY,
    NEW.created_at = NOW (),
    END;