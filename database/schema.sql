-- Users table
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    username VARCHAR(255) UNIQUE NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    display_name VARCHAR(255) NOT NULL,
    password_hash TEXT NOT NULL,
    role TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'Pending',
    created TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated TIMESTAMPTZ NOT NULL DEFAULT NOW()
);


-- Index on email for faster lookups
CREATE INDEX idx_users_email ON users(email);

-- Index on username for faster lookups
CREATE INDEX idx_users_username ON users(username);

-- Index on created for ordering
CREATE INDEX idx_users_created ON users(created);