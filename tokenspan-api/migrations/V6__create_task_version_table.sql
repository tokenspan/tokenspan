-- Add up migration script here

CREATE TYPE task_version_status AS ENUM ('draft', 'published', 'archived');

CREATE TABLE task_versions
(
    id           uuid PRIMARY KEY,
    semver       TEXT                NOT NULL,
    version      INT                 NOT NULL,
    release_note TEXT,
    description  TEXT,
    document     TEXT,
    status       task_version_status NOT NULL,
    task_id      uuid                NOT NULL,
    owner_id     uuid                NOT NULL,
    messages     jsonb[],
    created_at   TIMESTAMP           NOT NULL,
    updated_at   TIMESTAMP           NOT NULL,

    CONSTRAINT fk_task_versions_task_id FOREIGN KEY (task_id) REFERENCES tasks (id) ON DELETE CASCADE,
    CONSTRAINT fk_task_versions_owner_id FOREIGN KEY (owner_id) REFERENCES users (id)
);

CREATE INDEX idx_task_versions_task_id ON task_versions (task_id);
CREATE INDEX idx_task_versions_owner_id ON task_versions (owner_id);
CREATE UNIQUE INDEX idx_task_versions_version ON task_versions (id, version);
CREATE UNIQUE INDEX idx_task_versions_semver ON task_versions (id, semver);
CREATE INDEX idx_task_versions_created_at ON task_versions (created_at);