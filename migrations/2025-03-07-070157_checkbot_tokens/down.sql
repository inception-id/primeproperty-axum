-- This file should undo anything in `up.sql`
ALTER TABLE checkbot
DROP COLUMN input_tokens;

ALTER TABLE checkbot
DROP COLUMN output_tokens;

ALTER TABLE checkbot
DROP COLUMN total_tokens;

ALTER TABLE checkbot
DROP COLUMN temperature;
