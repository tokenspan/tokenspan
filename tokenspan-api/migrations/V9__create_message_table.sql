CREATE TABLE IF NOT EXISTS messages
(
    id                uuid PRIMARY KEY,
    thread_version_id uuid      NOT NULL,
    owner_id          uuid      NOT NULL,
    content           TEXT      NOT NULL,
    role              TEXT      NOT NULL,
    raw               TEXT      NOT NULL,
    index             INTEGER   NOT NULL,
    created_at        TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at        TIMESTAMP NOT NULL DEFAULT NOW(),

    CONSTRAINT fk_messages_owner_id FOREIGN KEY (owner_id) REFERENCES users (id) ON DELETE CASCADE
);

ALTER TABLE messages
    ADD search tsvector GENERATED ALWAYS AS
        (TO_TSVECTOR('simple', content) || ' ') STORED;

CREATE INDEX IF NOT EXISTS idx_message_thread_version_id ON messages (thread_version_id);
CREATE INDEX IF NOT EXISTS idx_message_owner_id ON messages (owner_id);
CREATE INDEX IF NOT EXISTS idx_message_created_at ON messages (created_at, id);
CREATE INDEX idx_message_search ON messages USING GIN(search);