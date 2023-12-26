-- Add down migration script here

DROP INDEX IF EXISTS idx_api_keys_owner_id;
DROP INDEX IF EXISTS idx_api_keys_provider_id;

DROP TABLE IF EXISTS api_keys;