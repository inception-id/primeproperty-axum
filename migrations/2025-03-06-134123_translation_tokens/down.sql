-- This file should undo anything in `up.sql`
ALTER TABLE translation
DROP COLUMN input_tokens;

ALTER TABLE translation
DROP COLUMN output_tokens;

ALTER TABLE translation
DROP COLUMN total_tokens;

ALTER TABLE translation
DROP COLUMN temperature;
