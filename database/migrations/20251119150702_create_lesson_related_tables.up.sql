-- create "lessons" table
CREATE TABLE "lessons" (
  "id" uuid NOT NULL DEFAULT gen_random_uuid(),
  "user_id" uuid NOT NULL,
  "course" character varying(255) NOT NULL,
  "unit" character varying(255) NOT NULL,
  "lesson" character varying(255) NOT NULL,
  "description" text NULL,
  "background" character varying(500) NULL,
  "word_sentences" uuid[] NULL DEFAULT '{}',
  "created" timestamptz NOT NULL DEFAULT now(),
  "updated" timestamptz NOT NULL DEFAULT now(),
  PRIMARY KEY ("id")
);
-- create index "idx_lessons_course" to table: "lessons"
CREATE INDEX "idx_lessons_course" ON "lessons" ("course");
-- create index "idx_lessons_created" to table: "lessons"
CREATE INDEX "idx_lessons_created" ON "lessons" ("created");
-- create index "idx_lessons_fts" to table: "lessons"
CREATE INDEX "idx_lessons_fts" ON "lessons" USING gin ((to_tsvector('english'::regconfig, (((((((course)::text || ' '::text) || (unit)::text) || ' '::text) || (lesson)::text) || ' '::text) || COALESCE(description, ''::text)))));
-- create index "idx_lessons_unit" to table: "lessons"
CREATE INDEX "idx_lessons_unit" ON "lessons" ("unit");
-- create index "idx_lessons_user_id" to table: "lessons"
CREATE INDEX "idx_lessons_user_id" ON "lessons" ("user_id");
-- create index "idx_lessons_word_sentences" to table: "lessons"
CREATE INDEX "idx_lessons_word_sentences" ON "lessons" USING gin ("word_sentences");
-- create "sentences" table
CREATE TABLE "sentences" (
  "id" uuid NOT NULL DEFAULT gen_random_uuid(),
  "sentence" text NOT NULL,
  "vietnamese_translation" text NOT NULL,
  "audio_url" character varying(500) NULL,
  "created" timestamptz NOT NULL DEFAULT now(),
  "updated" timestamptz NOT NULL DEFAULT now(),
  PRIMARY KEY ("id")
);
-- create index "idx_sentences_created" to table: "sentences"
CREATE INDEX "idx_sentences_created" ON "sentences" ("created");
-- create index "idx_sentences_sentence_fts" to table: "sentences"
CREATE INDEX "idx_sentences_sentence_fts" ON "sentences" USING gin ((to_tsvector('english'::regconfig, sentence)));
-- create "words" table
CREATE TABLE "words" (
  "id" uuid NOT NULL DEFAULT gen_random_uuid(),
  "word" character varying(255) NOT NULL,
  "phonics" character varying(255) NULL,
  "part_of_speech" character varying(50) NULL,
  "vietnamese_meaning" text NOT NULL,
  "image_url" character varying(500) NULL,
  "word_audio_url" character varying(500) NULL,
  "created" timestamptz NOT NULL DEFAULT now(),
  "updated" timestamptz NOT NULL DEFAULT now(),
  PRIMARY KEY ("id")
);
-- create index "idx_words_created" to table: "words"
CREATE INDEX "idx_words_created" ON "words" ("created");
-- create index "idx_words_part_of_speech" to table: "words"
CREATE INDEX "idx_words_part_of_speech" ON "words" ("part_of_speech");
-- create index "idx_words_word" to table: "words"
CREATE INDEX "idx_words_word" ON "words" ("word");
-- create "word_sentences" table
CREATE TABLE "word_sentences" (
  "id" uuid NOT NULL DEFAULT gen_random_uuid(),
  "word_id" uuid NOT NULL,
  "sentence_id" uuid NOT NULL,
  "created" timestamptz NOT NULL DEFAULT now(),
  PRIMARY KEY ("id"),
  CONSTRAINT "fk_word_sentences_sentence_id" FOREIGN KEY ("sentence_id") REFERENCES "sentences" ("id") ON UPDATE NO ACTION ON DELETE CASCADE,
  CONSTRAINT "fk_word_sentences_word_id" FOREIGN KEY ("word_id") REFERENCES "words" ("id") ON UPDATE NO ACTION ON DELETE CASCADE
);
-- create index "idx_word_sentences_sentence" to table: "word_sentences"
CREATE INDEX "idx_word_sentences_sentence" ON "word_sentences" ("sentence_id");
-- create index "idx_word_sentences_word" to table: "word_sentences"
CREATE INDEX "idx_word_sentences_word" ON "word_sentences" ("word_id");
