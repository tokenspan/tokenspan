CREATE TABLE IF NOT EXISTS functions
(
    id          uuid PRIMARY KEY,
    name        TEXT      NOT NULL,
    owner_id    uuid      NOT NULL,
    description TEXT      NOT NULL,
    parameters  jsonb     NOT NULL,
    response    jsonb,
    created_at  TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at  TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_created_at ON functions (created_at);