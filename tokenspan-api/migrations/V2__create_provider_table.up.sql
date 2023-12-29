-- Add up migration script here

-- create provider table
CREATE TABLE IF NOT EXISTS providers
(
    id         uuid PRIMARY KEY,
    name       TEXT      NOT NULL,
    slug       TEXT      NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE UNIQUE INDEX idx_provider_slug ON providers (slug);
CREATE INDEX idx_provider_created_at ON providers (created_at);