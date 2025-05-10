-- This file should undo anything in `up.sql`
ALTER TABLE properties
DROP COLUMN configurations;

ALTER TABLE properties
DROP COLUMN sold_channel;

DROP TYPE sold_channel;
