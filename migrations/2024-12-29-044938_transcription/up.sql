-- Your SQL goes here
CREATE TABLE speech_to_text (
                                id SERIAL PRIMARY KEY NOT NULL,
                                user_id uuid NOT NULL REFERENCES users(id),
                                created_at TIMESTAMP NOT NULL DEFAULT NOW(),
                                updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
                                audio_url VARCHAR,
                                transcription_text TEXT NOT NULL
);
SELECT diesel_manage_updated_at('speech_to_text');
