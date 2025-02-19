-- Your SQL goes here
ALTER TABLE translation_storage DROP COLUMN visibility;
ALTER TABLE checkbot_storage DROP COLUMN visibility;
ALTER TABLE speech_to_text_storage DROP COLUMN visibility;
ALTER TABLE text_to_speech_storage DROP COLUMN visibility;
DROP TYPE storage_visibility;

CREATE TYPE shared_storage_permission AS ENUM('view', 'edit');
CREATE TABLE shared_translation_storage (
    id SERIAL PRIMARY KEY NOT NULL,
    user_id uuid NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    shared_user_id uuid REFERENCES users(id) ON DELETE CASCADE ,
    translation_storage_id integer NOT NULL REFERENCES translation_storage(id) ON DELETE CASCADE,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    user_email varchar NOT NULL,
    shared_user_email varchar NOT NULL,
    permission shared_storage_permission NOT NULL DEFAULT 'view'
);
SELECT diesel_manage_updated_at('shared_translation_storage');
