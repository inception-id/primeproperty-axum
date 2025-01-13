-- Your SQL goes here
-- Your SQL goes here
CREATE TABLE text_to_speech_storage (
                                id SERIAL PRIMARY KEY NOT NULL,
                                user_id uuid NOT NULL REFERENCES users(id),
                                text_to_speech_id INT NOT NULL REFERENCES text_to_speech(id),
                                created_at TIMESTAMP NOT NULL DEFAULT NOW(),
                                updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
                                input_content TEXT NOT NULL,
                                audio_url VARCHAR NOT NULL,
                                voice VARCHAR NOT NULL
);
SELECT diesel_manage_updated_at('text_to_speech_storage');
