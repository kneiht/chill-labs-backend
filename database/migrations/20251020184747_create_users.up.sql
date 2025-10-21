-- create "users" table
CREATE TABLE "users" (
  "id" uuid NOT NULL DEFAULT gen_random_uuid(),
  "username" character varying(255) NOT NULL,
  "email" character varying(255) NULL,
  "display_name" character varying(255) NOT NULL,
  "password_hash" text NOT NULL,
  "role" text NOT NULL,
  "status" text NOT NULL DEFAULT 'Pending',
  "created" timestamptz NOT NULL DEFAULT now(),
  "updated" timestamptz NOT NULL DEFAULT now(),
  PRIMARY KEY ("id"),
  CONSTRAINT "users_email_key" UNIQUE ("email"),
  CONSTRAINT "users_username_key" UNIQUE ("username")
);
-- create index "idx_users_created" to table: "users"
CREATE INDEX "idx_users_created" ON "users" ("created");
-- create index "idx_users_email" to table: "users"
CREATE INDEX "idx_users_email" ON "users" ("email");
-- create index "idx_users_username" to table: "users"
CREATE INDEX "idx_users_username" ON "users" ("username");
