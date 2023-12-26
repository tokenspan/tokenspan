-- Add down migration script here

DROP INDEX IF EXISTS idx_model_slug;
DROP INDEX IF EXISTS idx_model_created_at;

DROP TABLE IF EXISTS models;