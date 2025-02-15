-- This file should undo anything in `up.sql`
ALTER TABLE translation_storage DROP COLUMN title;
ALTER TABLE translation_storage DROP COLUMN visibility;
ALTER TABLE checkbot_storage DROP COLUMN title;
ALTER TABLE checkbot_storage DROP COLUMN visibility;
ALTER TABLE speech_to_text_storage DROP COLUMN title;
ALTER TABLE speech_to_text_storage DROP COLUMN visibility;
ALTER TABLE text_to_speech_storage DROP COLUMN title;
ALTER TABLE text_to_speech_storage DROP COLUMN visibility;
DROP TYPE storage_visibility;
