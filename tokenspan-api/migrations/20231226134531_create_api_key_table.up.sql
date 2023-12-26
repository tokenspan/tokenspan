-- Add up migration script here

CREATE TABLE api_keys
(
    id          uuid PRIMARY KEY,
    name        TEXT      NOT NULL,
    key         TEXT      NOT NULL,
    owner_id    uuid      NOT NULL,
    provider_id uuid      NOT NULL,
    created_at  TIMESTAMP NOT NULL,
    updated_at  TIMESTAMP NOT NULL,

    CONSTRAINT fk_api_keys_owner_id FOREIGN KEY (owner_id) REFERENCES users (id),
    CONSTRAINT fk_api_keys_provider_id FOREIGN KEY (provider_id) REFERENCES providers (id)
);

CREATE INDEX idx_api_keys_owner_id ON api_keys (owner_id);
CREATE INDEX idx_api_keys_provider_id ON api_keys (provider_id);