-- Your SQL goes here
-- Your SQL goes here
CREATE TABLE checkbot (
                             id SERIAL PRIMARY KEY NOT NULL,
                             user_id uuid NOT NULL REFERENCES users(id),
                             created_at TIMESTAMP NOT NULL DEFAULT NOW(),
                             updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
                            instruction VARCHAR NOT NULL,
                             ai_system_prompt VARCHAR NOT NULL,
                             content TEXT NOT NULL,
                             completion TEXT NOT NULL,
                             updated_completion TEXT NOT NULL
);
SELECT diesel_manage_updated_at('checkbot');
