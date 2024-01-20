-- Add up migration script here

CREATE TYPE execution_status AS ENUM ('pending', 'running', 'success', 'failed');

CREATE TABLE executions
(
    id                uuid PRIMARY KEY,
    thread_version_id uuid             NOT NULL,
    executed_by_id    uuid             NOT NULL,
    parameter         jsonb            NOT NULL,
    elapsed           jsonb            NOT NULL,
    input_messages    jsonb[]          NOT NULL,
    output_messages   jsonb[]          NOT NULL,
    response          jsonb,
    error             jsonb,
    usage             jsonb,
    status            execution_status NOT NULL,
    created_at        TIMESTAMP        NOT NULL,
    updated_at        TIMESTAMP        NOT NULL,

    CONSTRAINT fk_executions_thread_version_id FOREIGN KEY (thread_version_id) REFERENCES thread_versions (id),
    CONSTRAINT fk_executions_executed_by_id FOREIGN KEY (executed_by_id) REFERENCES users (id)
);

CREATE INDEX idx_execution_thread_version_id ON executions (thread_version_id);
CREATE INDEX idx_execution_executed_by_id ON executions (executed_by_id);
CREATE INDEX idx_execution_created_at ON executions (created_at, id);