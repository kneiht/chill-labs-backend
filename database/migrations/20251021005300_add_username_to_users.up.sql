-- Add username column to users table as nullable first
ALTER TABLE users ADD COLUMN username VARCHAR(255);

-- Set username to email for existing users (as default)
UPDATE users SET username = email;

-- Now make the username column NOT NULL and UNIQUE
ALTER TABLE users ALTER COLUMN username SET NOT NULL;
ALTER TABLE users ADD CONSTRAINT users_username_key UNIQUE (username);

-- Create index on username for faster lookups
CREATE INDEX idx_users_username ON users(username);