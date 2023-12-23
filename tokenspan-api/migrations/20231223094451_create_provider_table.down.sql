-- Add down migration script here

DROP INDEX IF EXISTS idx_provider_created_at;

DROP TABLE IF EXISTS providers;