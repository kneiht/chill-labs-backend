-- Remove username column from users table
DROP INDEX IF EXISTS idx_users_username;
ALTER TABLE users DROP COLUMN username;