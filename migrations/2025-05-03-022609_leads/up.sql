-- Your SQL goes here
CREATE TABLE leads (
    id SERIAL PRIMARY KEY,
    user_id uuid NOT NULL REFERENCES agents (id) ON DELETE CASCADE,
    property_id INTEGER NOT NULL REFERENCES properties (id) ON DELETE CASCADE,
    created_at TIMESTAMP NOT NULL DEFAULT NOW (),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW (),
    name VARCHAR(255) NOT NULL,
    phone VARCHAR(255) NOT NULL,
    email VARCHAR(255),
    is_deleted BOOLEAN NOT NULL DEFAULT FALSE
);

SELECT
    diesel_manage_updated_at ('leads');
