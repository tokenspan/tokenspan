-- Add up migration script here

CREATE TABLE parameters
(
    id                uuid PRIMARY KEY,
    name              TEXT      NOT NULL,
    temperature       FLOAT     NOT NULL,
    max_tokens        INT       NOT NULL,
    stop_sequences    TEXT[]    NOT NULL,
    top_p             FLOAT     NOT NULL,
    frequency_penalty FLOAT     NOT NULL,
    presence_penalty  FLOAT     NOT NULL,
    extra             jsonb,
    model_id          uuid      NOT NULL,
    task_version_id   uuid      NOT NULL,
    created_at        TIMESTAMP NOT NULL,
    updated_at        TIMESTAMP NOT NULL,

    CONSTRAINT fk_parameters_model_id FOREIGN KEY (model_id) REFERENCES models (id),
    CONSTRAINT fk_parameters_task_version_id FOREIGN KEY (task_version_id) REFERENCES task_versions (id)
);

CREATE INDEX idx_parameters_model_id ON parameters (model_id);
CREATE INDEX idx_parameters_task_version_id ON parameters (task_version_id);
CREATE INDEX idx_parameters_created_at ON parameters (created_at);