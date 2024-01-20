-- Add up migration script here

CREATE TABLE threads
(
    id         uuid PRIMARY KEY,
    name       TEXT      NOT NULL,
    slug       TEXT      NOT NULL,
    owner_id   uuid      NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL,

    CONSTRAINT fk_threads_owner_id FOREIGN KEY (owner_id) REFERENCES users (id)
);

ALTER TABLE threads
    ADD search tsvector GENERATED ALWAYS AS
        (TO_TSVECTOR('simple', name) || ' ') STORED;

CREATE UNIQUE INDEX idx_thread_slug ON threads (slug);
CREATE INDEX idx_thread_owner_id ON threads (owner_id);
CREATE INDEX idx_thread_created_at ON threads (created_at, id);
CREATE INDEX idx_thread_search ON threads USING GIN(search);