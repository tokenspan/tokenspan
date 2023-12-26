-- Add down migration script here

DROP INDEX IF EXISTS idx_parameters_model_id;
DROP INDEX IF EXISTS idx_parameters_task_version_id;
DROP INDEX IF EXISTS idx_parameters_created_at;

DROP TABLE IF EXISTS parameters;