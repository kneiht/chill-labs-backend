-- reverse: modify "users" table
ALTER TABLE "users" ALTER COLUMN "display_name" SET NOT NULL, ALTER COLUMN "username" SET NOT NULL;
