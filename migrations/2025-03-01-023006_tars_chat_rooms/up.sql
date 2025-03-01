-- Your SQL goes here
CREATE TABLE tars_chat_rooms (
    id SERIAL PRIMARY KEY NOT NULL,
    ai_model_id INTEGER NOT NULL REFERENCES ai_models (id),
    user_id uuid NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    created_at TIMESTAMP NOT NULL DEFAULT NOW (),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW (),
    title VARCHAR,
    is_deleted BOOLEAN NOT NULL DEFAULT FALSE
);

SELECT
    diesel_manage_updated_at ('tars_chat_rooms');
