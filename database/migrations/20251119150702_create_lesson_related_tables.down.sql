-- reverse: create index "idx_word_sentences_word" to table: "word_sentences"
DROP INDEX "idx_word_sentences_word";
-- reverse: create index "idx_word_sentences_sentence" to table: "word_sentences"
DROP INDEX "idx_word_sentences_sentence";
-- reverse: create "word_sentences" table
DROP TABLE "word_sentences";
-- reverse: create index "idx_words_word" to table: "words"
DROP INDEX "idx_words_word";
-- reverse: create index "idx_words_part_of_speech" to table: "words"
DROP INDEX "idx_words_part_of_speech";
-- reverse: create index "idx_words_created" to table: "words"
DROP INDEX "idx_words_created";
-- reverse: create "words" table
DROP TABLE "words";
-- reverse: create index "idx_sentences_sentence_fts" to table: "sentences"
DROP INDEX "idx_sentences_sentence_fts";
-- reverse: create index "idx_sentences_created" to table: "sentences"
DROP INDEX "idx_sentences_created";
-- reverse: create "sentences" table
DROP TABLE "sentences";
-- reverse: create index "idx_lessons_word_sentences" to table: "lessons"
DROP INDEX "idx_lessons_word_sentences";
-- reverse: create index "idx_lessons_user_id" to table: "lessons"
DROP INDEX "idx_lessons_user_id";
-- reverse: create index "idx_lessons_unit" to table: "lessons"
DROP INDEX "idx_lessons_unit";
-- reverse: create index "idx_lessons_fts" to table: "lessons"
DROP INDEX "idx_lessons_fts";
-- reverse: create index "idx_lessons_created" to table: "lessons"
DROP INDEX "idx_lessons_created";
-- reverse: create index "idx_lessons_course" to table: "lessons"
DROP INDEX "idx_lessons_course";
-- reverse: create "lessons" table
DROP TABLE "lessons";
