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

CREATE UNIQUE INDEX idx_threads_slug ON threads (slug);
CREATE INDEX idx_threads_owner_id ON threads (owner_id);