-- Your SQL goes here
CREATE TABLE
    chats (
        id BIGINT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
        ulid CHAR(26),
        name VARCHAR(255),
        description LONGTEXT,
        created_by BIGINT UNSIGNED,
        -- a date string, note that this is not the appropriate data type for a date, but the MySQL driver for WASI & Rust doesn't support the date type
        created_at VARCHAR(255),
        updated_at VARCHAR(255)
    );

ALTER TABLE chats ADD FOREIGN KEY (created_by) REFERENCES sessions (id);

ALTER TABLE chats ADD INDEX (ulid);

-- this is needed for the MySQL driver for WASI & Rust to work as it doesn't support the date type, kinda hacky..
CREATE TRIGGER chats_before_insert BEFORE INSERT ON chats FOR EACH ROW BEGIN
SET
    NEW.created_at = NOW(),
    NEW.updated_at = NOW();

END;

CREATE TRIGGER chats_before_update BEFORE
UPDATE ON chats FOR EACH ROW BEGIN
SET
    NEW.updated_at = NOW();

END;