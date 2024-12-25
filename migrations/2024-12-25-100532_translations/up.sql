-- Your SQL goes here
CREATE TABLE translation (
                             id SERIAL PRIMARY KEY NOT NULL,
                             user_id uuid NOT NULL REFERENCES users(id),
                             created_at TIMESTAMP NOT NULL DEFAULT NOW(),
                             updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
                             ai_system_prompt VARCHAR NOT NULL,
                             content_language VARCHAR,
                             target_language VARCHAR NOT NULL,
                             content TEXT NOT NULL,
                             completion TEXT NOT NULL,
                             updated_completion TEXT NOT NULL
);
SELECT diesel_manage_updated_at('translation');
