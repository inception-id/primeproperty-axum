-- This file should undo anything in `up.sql`
ALTER TABLE speech_to_text ALTER COLUMN audio_url DROP NOT NULL;
ALTER TABLE speech_to_text DROP COLUMN language;
