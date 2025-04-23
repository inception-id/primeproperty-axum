-- Your SQL goes here
CREATE TYPE purchase_status AS ENUM ('for_sale', 'for_rent', 'for_sale_or_rent');

CREATE TYPE sold_status AS ENUM ('available', 'sold');

CREATE TYPE building_condition AS ENUM (
    'new',
    'good',
    'renovated',
    'renovation_required',
    'old'
);

CREATE TYPE furniture_capacity AS ENUM ('furnished', 'semi_furnished', 'unfurnished');

CREATE TABLE properties (
    id SERIAL PRIMARY KEY,
    agent_id uuid NOT NULL REFERENCES agents (id),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    title VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    address TEXT NOT NULL,
    gmap_iframe TEXT,
    price INTEGER NOT NULL,
    images JSONB NOT NULL DEFAULT '[]',
    purchase_status purchase_status NOT NULL,
    sold_status sold_status NOT NULL DEFAULT 'available',
    land_area INTEGER,
    land_width INTEGER,
    land_length INTEGER,
    building_type VARCHAR(255) NOT NULL,
    building_condition building_condition NOT NULL,
    building_furniture_capacity furniture_capacity,
    building_certificate VARCHAR(255) NOT NULL,
    building_levels INTEGER NOT NULL DEFAULT 1,
    building_area INTEGER,
    building_width INTEGER,
    building_length INTEGER,
    building_height INTEGER,
    bedrooms_count INTEGER,
    bathrooms_count INTEGER,
    garage_capacity INTEGER,
    carport_capacity INTEGER,
    electrical_power INTEGER,
    facilities JSONB NOT NULL DEFAULT '[]'
);

SELECT
    diesel_manage_updated_at ('properties');
