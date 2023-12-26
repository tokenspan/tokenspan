-- Add down migration script here

DROP INDEX IF EXISTS idx_executions_task_version_id;
DROP INDEX IF EXISTS idx_executions_executed_by_id;
DROP INDEX IF EXISTS idx_executions_parameter_id;
DROP INDEX IF EXISTS idx_executions_created_at;

DROP TABLE IF EXISTS executions;

DROP TYPE IF EXISTS execution_status;