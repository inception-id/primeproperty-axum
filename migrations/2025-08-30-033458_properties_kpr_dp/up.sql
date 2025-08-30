-- Your SQL goes here
ALTER TABLE properties
ADD COLUMN description_seo VARCHAR(255);

ALTER TABLE properties
ADD COLUMN price_down_payment BIGINT;
