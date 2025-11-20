-- reverse: rename a column from "word_audio_url" to "audio_url"
ALTER TABLE "words" RENAME COLUMN "audio_url" TO "word_audio_url";
-- reverse: rename a column from "vietnamese_meaning" to "meaning"
ALTER TABLE "words" RENAME COLUMN "meaning" TO "vietnamese_meaning";
-- reverse: create index "idx_words_word_lower" to table: "words"
DROP INDEX "idx_words_word_lower";
-- reverse: modify "words" table
ALTER TABLE "words" DROP COLUMN "user_id";
-- reverse: drop index "idx_words_word" from table: "words"
CREATE INDEX "idx_words_word" ON "words" ("word");
-- reverse: drop index "idx_words_part_of_speech" from table: "words"
CREATE INDEX "idx_words_part_of_speech" ON "words" ("part_of_speech");
-- reverse: drop index "idx_words_created" from table: "words"
CREATE INDEX "idx_words_created" ON "words" ("created");
-- reverse: modify "word_sentences" table
ALTER TABLE "word_sentences" DROP COLUMN "updated";
-- reverse: rename a column from "vietnamese_translation" to "translation"
ALTER TABLE "sentences" RENAME COLUMN "translation" TO "vietnamese_translation";
-- reverse: modify "sentences" table
ALTER TABLE "sentences" DROP COLUMN "user_id";
-- reverse: drop index "idx_sentences_created" from table: "sentences"
CREATE INDEX "idx_sentences_created" ON "sentences" ("created");
