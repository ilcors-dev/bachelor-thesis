-- Your SQL goes here
CREATE TABLE
    messages (
        id BIGINT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
        ulid CHAR(26),
        text LONGTEXT,
        created_at VARCHAR(255),
        updated_at VARCHAR(255)
    );

ALTER TABLE messages ADD INDEX (ulid);

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