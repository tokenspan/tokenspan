-- Add up migration script here

CREATE TABLE parameters
(
    id                uuid PRIMARY KEY,
    name              TEXT      NOT NULL,
    temperature       float4    NOT NULL,
    max_tokens        INT       NOT NULL,
    stop_sequences    TEXT[]    NOT NULL,
    top_p             float4    NOT NULL,
    frequency_penalty float4    NOT NULL,
    presence_penalty  float4    NOT NULL,
    extra             jsonb,
    model_id          uuid      NOT NULL,
    thread_version_id uuid      NOT NULL,
    is_default        BOOLEAN   NOT NULL DEFAULT FALSE,
    created_at        TIMESTAMP NOT NULL,
    updated_at        TIMESTAMP NOT NULL,

    CONSTRAINT fk_parameters_model_id FOREIGN KEY (model_id) REFERENCES models (id),
    CONSTRAINT fk_parameters_thread_version_id FOREIGN KEY (thread_version_id) REFERENCES thread_versions (id) ON DELETE CASCADE
);

CREATE INDEX idx_parameters_model_id ON parameters (model_id);
CREATE INDEX idx_parameters_thread_version_id ON parameters (thread_version_id);
CREATE INDEX idx_parameters_created_at ON parameters (created_at);