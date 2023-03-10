-- Your SQL goes here
CREATE TABLE
    messages (
        id BIGINT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
        chat_id BIGINT UNSIGNED NOT NULL ON DELETE CASCADE,
        sender_id BIGINT UNSIGNED NOT NULL ON DELETE SET NULL,
        ulid CHAR(26),
        text LONGTEXT,
        -- a date string, note that this is not the appropriate data type for a date, but the MySQL driver for WASI & Rust doesn't support the date type
        created_at VARCHAR(255),
        updated_at VARCHAR(255)
    );

ALTER TABLE messages ADD FOREIGN KEY (chat_id) REFERENCES chats (id);

ALTER TABLE messages ADD FOREIGN KEY (sender_id) REFERENCES sessions (id);

ALTER TABLE messages ADD INDEX (ulid);

-- this is needed for the MySQL driver for WASI & Rust to work as it doesn't support the date type, kinda hacky..
CREATE TRIGGER messages_before_insert BEFORE INSERT ON messages FOR EACH ROW BEGIN
SET
    NEW.created_at = NOW(),
    NEW.updated_at = NOW();

END;

CREATE TRIGGER messages_before_update BEFORE
UPDATE ON messages FOR EACH ROW BEGIN
SET
    NEW.updated_at = NOW();

END;