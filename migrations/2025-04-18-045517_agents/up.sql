-- Your SQL goes here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TYPE agent_role AS ENUM ('admin', 'agent');

CREATE TABLE agents (
    id uuid PRIMARY KEY DEFAULT uuid_generate_v4 (),
    supertokens_user_id VARCHAR,
    created_at TIMESTAMP NOT NULL DEFAULT NOW (),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW (),
    email VARCHAR NOT NULL,
    phone_number VARCHAR NOT NULL,
    role agent_role NOT NULL DEFAULT 'agent'
);

SELECT
    diesel_manage_updated_at ('agents');
