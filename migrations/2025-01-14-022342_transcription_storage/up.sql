-- Your SQL goes here
CREATE TABLE speech_to_text_storage (
                                id SERIAL PRIMARY KEY NOT NULL,
                                user_id uuid NOT NULL REFERENCES users(id),
                                speech_to_text_id INT NOT NULL REFERENCES speech_to_text(id),
                                created_at TIMESTAMP NOT NULL DEFAULT NOW(),
                                updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
                                audio_url VARCHAR NOT NULL,
                                updated_transcription_text TEXT NOT NULL,
                                language VARCHAR
);
SELECT diesel_manage_updated_at('speech_to_text_storage');
