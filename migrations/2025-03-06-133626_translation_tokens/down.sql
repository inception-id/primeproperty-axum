-- This file should undo anything in `up.sql`
CREATE TYPE shared_storage_permission AS ENUM ('view', 'edit');

CREATE TABLE shared_translation_storage (
    id SERIAL PRIMARY KEY NOT NULL,
    user_id uuid NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    shared_user_id uuid REFERENCES users (id) ON DELETE CASCADE,
    translation_storage_id integer NOT NULL REFERENCES translation_storage (id) ON DELETE CASCADE,
    created_at TIMESTAMP NOT NULL DEFAULT NOW (),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW (),
    user_email varchar NOT NULL,
    shared_user_email varchar NOT NULL,
    permission shared_storage_permission NOT NULL DEFAULT 'view'
);

SELECT
    diesel_manage_updated_at ('shared_translation_storage');

CREATE TABLE ai_models (
    id SERIAL PRIMARY KEY NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW (),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW (),
    parent VARCHAR(255) NOT NULL,
    label VARCHAR(255) NOT NULL,
    value VARCHAR(255) NOT NULL
);

SELECT
    diesel_manage_updated_at ('ai_models');

CREATE TABLE tars_chat_rooms (
    id SERIAL PRIMARY KEY NOT NULL,
    ai_model_id INTEGER NOT NULL REFERENCES ai_models (id),
    user_id uuid NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    created_at TIMESTAMP NOT NULL DEFAULT NOW (),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW (),
    title VARCHAR,
    is_temporary BOOLEAN NOT NULL DEFAULT TRUE
);

SELECT
    diesel_manage_updated_at ('tars_chat_rooms');

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
