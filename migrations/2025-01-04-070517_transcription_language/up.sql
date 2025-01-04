-- Your SQL goes here
ALTER TABLE speech_to_text ALTER COLUMN audio_url SET NOT NULL;
ALTER TABLE speech_to_text ADD COLUMN language VARCHAR;