-- Add up migration script here

-- create model table
CREATE TABLE models
(
    id             uuid PRIMARY KEY,
    name           TEXT      NOT NULL,
    description    TEXT      NOT NULL,
    slug           TEXT      NOT NULL,
    context        INTEGER   NOT NULL,
    input_pricing  jsonb     NOT NULL,
    output_pricing jsonb     NOT NULL,
    training_at    TIMESTAMP NOT NULL,
    provider_id    uuid      NOT NULL,
    created_at     TIMESTAMP NOT NULL,
    updated_at     TIMESTAMP NOT NULL,

    CONSTRAINT fk_models_provider_id FOREIGN KEY (provider_id) REFERENCES providers (id)
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_model_slug ON models (slug);
CREATE INDEX IF NOT EXISTS idx_model_created_at ON models (created_at, id);