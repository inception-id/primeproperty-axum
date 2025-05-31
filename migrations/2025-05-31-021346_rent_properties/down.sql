-- This file should undo anything in `up.sql`
ALTER TABLE properties
DROP COLUMN rent_time;

DROP TYPE rent_time_unit;

ALTER TABLE properties
DROP COLUMN currency;

DROP TYPE currency_unit;

ALTER TABLE properties
ALTER COLUMN building_certificate
DROP DEFAULT;
