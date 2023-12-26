-- Add up migration script here

CREATE TABLE tasks
(
    id         uuid PRIMARY KEY,
    name       TEXT      NOT NULL,
    slug       TEXT      NOT NULL,
    owner_id   uuid      NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL,

    CONSTRAINT fk_tasks_owner_id FOREIGN KEY (owner_id) REFERENCES users (id)
);

CREATE UNIQUE INDEX idx_tasks_slug ON tasks (slug);
CREATE INDEX idx_tasks_owner_id ON tasks (owner_id);