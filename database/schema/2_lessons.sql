-- Words table: Stores individual word
CREATE TABLE words (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    word VARCHAR(255) NOT NULL,
    phonics VARCHAR(255),
    part_of_speech VARCHAR(50),
    vietnamese_meaning TEXT NOT NULL,
    image_url VARCHAR(500),
    word_audio_url VARCHAR(500),
    created TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX idx_words_word ON words(word);
CREATE INDEX idx_words_part_of_speech ON words(part_of_speech);
CREATE INDEX idx_words_created ON words(created);




-- Sentences table: Stores example sentences
CREATE TABLE sentences (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    sentence TEXT NOT NULL,
    vietnamese_translation TEXT NOT NULL,
    audio_url VARCHAR(500),
    created TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX idx_sentences_created ON sentences(created);
-- GIN index for full-text search on sentence column
CREATE INDEX idx_sentences_sentence_fts ON sentences USING GIN (to_tsvector('english', sentence));



-- Word_Sentences table: Many-to-many relationship between words and sentences.
CREATE TABLE word_sentences (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    word_id UUID NOT NULL,
    sentence_id UUID NOT NULL,
    created TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT fk_word_sentences_word_id FOREIGN KEY (word_id) REFERENCES words(id) ON DELETE CASCADE,
    CONSTRAINT fk_word_sentences_sentence_id FOREIGN KEY (sentence_id) REFERENCES sentences(id) ON DELETE CASCADE
);
CREATE INDEX idx_word_sentences_word ON word_sentences(word_id);
CREATE INDEX idx_word_sentences_sentence ON word_sentences(sentence_id);



-- Lessons table: Stores lessons with embedded word-sentence relationships.
CREATE TABLE lessons (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL,
    course VARCHAR(255) NOT NULL,
    unit VARCHAR(255) NOT NULL,
    lesson VARCHAR(255) NOT NULL,
    description TEXT,
    background VARCHAR(500),
    word_sentences UUID[] DEFAULT '{}', -- Array of word_sentence IDs
    created TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX idx_lessons_user_id ON lessons(user_id);
CREATE INDEX idx_lessons_course ON lessons(course);
CREATE INDEX idx_lessons_unit ON lessons(unit);
CREATE INDEX idx_lessons_created ON lessons(created);
-- GIN index for word_sentences array for efficient querying
CREATE INDEX idx_lessons_word_sentences ON lessons USING GIN(word_sentences);
-- GIN index for full-text search on course, unit, lesson, and description
CREATE INDEX idx_lessons_fts ON lessons USING GIN (
    to_tsvector('english', course || ' ' || unit || ' ' || lesson || ' ' || COALESCE(description, ''))
);
