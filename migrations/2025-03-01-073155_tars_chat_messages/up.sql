-- Your SQL goes here
CREATE TYPE tars_chat_messages_role AS ENUM (
    'developer',
    'system',
    'user',
    'assistant',
    'tool'
);

CREATE TABLE tars_chat_messages (
    id SERIAL PRIMARY KEY NOT NULL,
    tars_chat_room_id INTEGER NOT NULL REFERENCES tars_chat_rooms (id) ON DELETE CASCADE,
    created_at TIMESTAMP NOT NULL DEFAULT NOW (),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW (),
    role tars_chat_messages_role NOT NULL,
    content TEXT NOT NULL,
    input_tokens INTEGER NOT NULL DEFAULT 0,
    output_tokens INTEGER NOT NULL DEFAULT 0,
    total_tokens INTEGER NOT NULL DEFAULT 0
);

SELECT
    diesel_manage_updated_at ('tars_chat_messages');
