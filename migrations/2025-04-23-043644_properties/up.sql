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
    user_id uuid NOT NULL REFERENCES agents (id) ON DELETE CASCADE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    site_path VARCHAR NOT NULL,
    title VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    province VARCHAR(255) NOT NULL,
    regency VARCHAR(255) NOT NULL,
    street VARCHAR(255) NOT NULL,
    gmap_iframe TEXT,
    price INTEGER NOT NULL,
    images JSONB NOT NULL DEFAULT '[]',
    purchase_status purchase_status NOT NULL,
    sold_status sold_status NOT NULL DEFAULT 'available',
    land_measurements JSONB NOT NULL DEFAULT '{}',
    building_type VARCHAR(255) NOT NULL,
    building_condition building_condition NOT NULL,
    building_furniture_capacity furniture_capacity,
    building_certificate VARCHAR(255) NOT NULL,
    building_measurements JSONB NOT NULL DEFAULT '{}',
    specifications JSONB NOT NULL DEFAULT '{}',
    facilities JSONB NOT NULL DEFAULT '[]'
);

SELECT
    diesel_manage_updated_at ('properties');
