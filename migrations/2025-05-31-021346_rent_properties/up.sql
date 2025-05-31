-- Your SQL goes here
ALTER TABLE properties
ALTER COLUMN building_certificate
SET DEFAULT 'lainnya';

CREATE TYPE currency_unit AS ENUM ('IDR', 'USD');

ALTER TABLE properties
ADD COLUMN currency currency_unit NOT NULL DEFAULT 'IDR';

CREATE TYPE rent_time_unit AS ENUM ('monthly', 'yearly');

ALTER TABLE properties
ADD COLUMN rent_time rent_time_unit;
