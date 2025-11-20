-- drop index "idx_sentences_created" from table: "sentences"
DROP INDEX "idx_sentences_created";
-- modify "sentences" table
ALTER TABLE "sentences" ADD COLUMN "user_id" uuid NOT NULL;
-- rename a column from "vietnamese_translation" to "translation"
ALTER TABLE "sentences" RENAME COLUMN "vietnamese_translation" TO "translation";
-- modify "word_sentences" table
ALTER TABLE "word_sentences" ADD COLUMN "updated" timestamptz NOT NULL DEFAULT now();
-- drop index "idx_words_created" from table: "words"
DROP INDEX "idx_words_created";
-- drop index "idx_words_part_of_speech" from table: "words"
DROP INDEX "idx_words_part_of_speech";
-- drop index "idx_words_word" from table: "words"
DROP INDEX "idx_words_word";
-- modify "words" table
ALTER TABLE "words" ADD COLUMN "user_id" uuid NOT NULL;
-- create index "idx_words_word_lower" to table: "words"
CREATE INDEX "idx_words_word_lower" ON "words" ((lower((word)::text)));
-- rename a column from "vietnamese_meaning" to "meaning"
ALTER TABLE "words" RENAME COLUMN "vietnamese_meaning" TO "meaning";
-- rename a column from "word_audio_url" to "audio_url"
ALTER TABLE "words" RENAME COLUMN "word_audio_url" TO "audio_url";
