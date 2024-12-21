-- Your SQL goes here
CREATE TABLE ai_system_prompts (
    id SERIAL PRIMARY KEY NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    product_name VARCHAR NOT NULL,
    prompt VARCHAR NOT NULL
)
