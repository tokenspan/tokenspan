-- Add down migration script here
DROP INDEX IF EXISTS idx_user_username;
DROP INDEX IF EXISTS idx_user_email;
DROP INDEX IF EXISTS idx_user_created_at;

DROP TABLE IF EXISTS users;
DROP TYPE IF EXISTS user_role;