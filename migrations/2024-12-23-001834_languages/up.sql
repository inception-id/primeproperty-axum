-- Your SQL goes here
-- Your SQL goes here
CREATE TABLE languages (
                                   id SERIAL PRIMARY KEY NOT NULL,
                                   created_at TIMESTAMP NOT NULL DEFAULT NOW(),
                                   updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
                                   title VARCHAR NOT NULL,
                                   iso_639_1 VARCHAR(2) NOT NULL
);
SELECT diesel_manage_updated_at('languages');
