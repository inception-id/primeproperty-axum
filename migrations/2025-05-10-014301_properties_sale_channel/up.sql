-- Your SQL goes here
CREATE TYPE sold_channel AS ENUM ('web', 'r123', 'socmed', 'banner', 'others');

ALTER TABLE properties
ADD COLUMN sold_channel sold_channel;

ALTER TABLE properties
ADD COLUMN configurations JSONB NOT NULL DEFAULT '{}';
