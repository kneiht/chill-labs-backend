-- Users table: Stores user information for the English coaching application.
-- Fields:
-- - id: Unique identifier for each user.
-- - username: Optional unique username.
-- - email: Optional unique email address.
-- - display_name: User's display name.
-- - password_hash: Hashed password for authentication.
-- - role: User role (e.g., Student, Teacher, Admin).
-- - status: User status (e.g., Active, Pending, Suspended).
-- - created: Timestamp when the user was created.
-- - updated: Timestamp when the user was last updated.
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    username VARCHAR(255) UNIQUE,
    email VARCHAR(255) UNIQUE,
    display_name VARCHAR(255),
    password_hash TEXT NOT NULL,
    role TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'Pending',
    created TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated TIMESTAMPTZ NOT NULL DEFAULT NOW()
);


-- Indexes for performance:
-- - idx_users_email: Speeds up email-based queries.
-- - idx_users_username: Speeds up username-based queries.
-- - idx_users_created: Speeds up ordering by creation date.
CREATE INDEX idx_users_email ON users(email);
CREATE INDEX idx_users_username ON users(username);
CREATE INDEX idx_users_created ON users(created);

-- Notes table: Stores notes for users in the coaching application.
-- Fields:
-- - id: Unique identifier for each note.
-- - user_id: ID of the associated user.
-- - title: Title of the note.
-- - content: The text content of the note.
-- - created: Timestamp when the note was created.
-- - updated: Timestamp when the note was last updated.
CREATE TABLE notes (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL,
    title VARCHAR(255) NOT NULL,
    content TEXT NOT NULL,
    created TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Indexes for notes table:
-- - idx_notes_user_id: Speeds up queries by user.
-- - idx_notes_created: Speeds up ordering by creation date.
CREATE INDEX idx_notes_user_id ON notes(user_id);
CREATE INDEX idx_notes_created ON notes(created);