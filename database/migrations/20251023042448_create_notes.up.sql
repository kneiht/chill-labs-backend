-- create "notes" table
CREATE TABLE "notes" (
  "id" uuid NOT NULL DEFAULT gen_random_uuid(),
  "user_id" uuid NOT NULL,
  "title" character varying(255) NOT NULL,
  "content" text NOT NULL,
  "created" timestamptz NOT NULL DEFAULT now(),
  "updated" timestamptz NOT NULL DEFAULT now(),
  PRIMARY KEY ("id")
);
-- create index "idx_notes_created" to table: "notes"
CREATE INDEX "idx_notes_created" ON "notes" ("created");
-- create index "idx_notes_user_id" to table: "notes"
CREATE INDEX "idx_notes_user_id" ON "notes" ("user_id");
