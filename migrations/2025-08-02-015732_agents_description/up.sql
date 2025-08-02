-- Your SQL goes here
UPDATE agents SET fullname = LOWER(fullname);

ALTER TABLE agents ADD COLUMN description VARCHAR;
