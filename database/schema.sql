-- Add uuid-ossp extension if not already added
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Create user_status enum if it doesn't exist
DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'user_status') THEN
        CREATE TYPE user_status AS ENUM ('active', 'pending', 'suspended');
    END IF;
END
$$;

-- Create user_role enum if it doesn't exist
DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'user_role') THEN
        CREATE TYPE user_role AS ENUM ('student', 'teacher', 'admin');
    END IF;
END
$$;

-- Create membership_type enum if it doesn't exist
DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'membership_type') THEN
        CREATE TYPE membership_type AS ENUM ('free', 'Premium', 'Trial');
    END IF;
END
$$;

-- Create gender_type enum if it doesn't exist
DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'gender_type') THEN
        CREATE TYPE gender_type AS ENUM ('male', 'female', 'other');
    END IF;
END
$$;


-- Create the users table
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    display_name TEXT NOT NULL,
    email TEXT NOT NULL UNIQUE,
    email_verified BOOLEAN NOT NULL DEFAULT FALSE,
    password_hash TEXT NOT NULL,
    status user_status NOT NULL DEFAULT 'pending',
    role user_role NOT NULL DEFAULT 'student',
    avatar_url TEXT,
    created TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated TIMESTAMPTZ NOT NULL DEFAULT now(),
    last_login TIMESTAMPTZ,
    membership membership_type NOT NULL DEFAULT 'free',
    gender gender_type NOT NULL DEFAULT 'other',
    date_of_birth DATE,
    phone TEXT,
    bio TEXT
);


-- Recreate any indexes, constraints, or foreign keys that were on the original table
CREATE INDEX idx_users_email ON users(email);



-- Example migration (e.g., ./migrations/YYYYMMDDHHMMSS_create_posts.sql)
CREATE TABLE posts (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    title VARCHAR(255) NOT NULL,
    content TEXT NOT NULL,
    -- author_id BIGINT NOT NULL REFERENCES users(id), -- If linking to users
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);





-- Create email_verification_tokens table
CREATE TABLE IF NOT EXISTS email_verification_tokens (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    token TEXT NOT NULL,
    expires_at TIMESTAMPTZ NOT NULL,
    created TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT unique_user_token UNIQUE (user_id, token)
);

-- Create index for faster token lookups
CREATE INDEX IF NOT EXISTS idx_email_verification_tokens_token ON email_verification_tokens(token);

-- Create index for faster user_id lookups
CREATE INDEX IF NOT EXISTS idx_email_verification_tokens_user_id ON email_verification_tokens(user_id);