-- Your SQL goes here
-- Your SQL goes here
CREATE TABLE translation_storage (
                             id SERIAL PRIMARY KEY NOT NULL,
                             user_id uuid NOT NULL REFERENCES users(id),
                            translation_id INTEGER NOT NULL REFERENCES translation(id),
                             created_at TIMESTAMP NOT NULL DEFAULT NOW(),
                             updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
                             content_language VARCHAR,
                             target_language VARCHAR NOT NULL,
                             content TEXT NOT NULL,
                             updated_completion TEXT NOT NULL
);
SELECT diesel_manage_updated_at('translation_storage');
