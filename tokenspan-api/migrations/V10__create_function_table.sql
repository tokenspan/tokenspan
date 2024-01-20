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

ALTER TABLE functions
    ADD search tsvector GENERATED ALWAYS AS
        (TO_TSVECTOR('simple', name) || ' ' || TO_TSVECTOR('simple', description) || ' ') STORED;

CREATE INDEX IF NOT EXISTS idx_function_created_at ON functions (created_at, id);
CREATE INDEX idx_function_search ON functions USING GIN(search);