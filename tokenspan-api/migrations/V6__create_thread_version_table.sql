-- Add up migration script here

CREATE TYPE thread_version_status AS ENUM ('draft', 'published', 'archived');

CREATE TABLE thread_versions
(
    id           uuid PRIMARY KEY,
    semver       TEXT                  NOT NULL,
    version      INT                   NOT NULL,
    release_note TEXT,
    description  TEXT,
    document     TEXT,
    status       thread_version_status NOT NULL,
    thread_id    uuid                  NOT NULL,
    owner_id     uuid                  NOT NULL,
    created_at   TIMESTAMP             NOT NULL,
    updated_at   TIMESTAMP             NOT NULL,

    CONSTRAINT fk_thread_versions_thread_id FOREIGN KEY (thread_id) REFERENCES threads (id) ON DELETE CASCADE,
    CONSTRAINT fk_thread_versions_owner_id FOREIGN KEY (owner_id) REFERENCES users (id)
);

CREATE INDEX idx_thread_versions_thread_id ON thread_versions (thread_id);
CREATE INDEX idx_thread_versions_owner_id ON thread_versions (owner_id);
CREATE UNIQUE INDEX idx_thread_versions_version ON thread_versions (id, version);
CREATE UNIQUE INDEX idx_thread_versions_semver ON thread_versions (id, semver);
CREATE INDEX idx_thread_versions_created_at ON thread_versions (created_at);