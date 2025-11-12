-- create "vocabs" table
CREATE TABLE "vocabs" (
  "id" uuid NOT NULL DEFAULT gen_random_uuid(),
  "user_id" uuid NOT NULL,
  "words" jsonb NOT NULL DEFAULT '[]',
  "created" timestamptz NOT NULL DEFAULT now(),
  "updated" timestamptz NOT NULL DEFAULT now(),
  PRIMARY KEY ("id")
);
-- create index "idx_vocabs_created" to table: "vocabs"
CREATE INDEX "idx_vocabs_created" ON "vocabs" ("created" DESC);
-- create index "idx_vocabs_updated" to table: "vocabs"
CREATE INDEX "idx_vocabs_updated" ON "vocabs" ("updated" DESC);
-- create index "idx_vocabs_user_id" to table: "vocabs"
CREATE INDEX "idx_vocabs_user_id" ON "vocabs" ("user_id");
