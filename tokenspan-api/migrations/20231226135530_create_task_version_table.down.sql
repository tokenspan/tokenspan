-- Add down migration script here

DROP INDEX IF EXISTS idx_task_versions_task_id;
DROP INDEX IF EXISTS idx_task_versions_owner_id;
DROP INDEX IF EXISTS idx_task_versions_version;
DROP INDEX IF EXISTS idx_task_versions_semver;
DROP INDEX IF EXISTS idx_task_versions_created_at;

DROP TABLE IF EXISTS task_versions;