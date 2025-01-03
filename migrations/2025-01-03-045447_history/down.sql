-- This file should undo anything in `up.sql`
ALTER TABLE checkbot ADD COLUMN updated_completion TEXT NOT NULL DEFAULT '';
ALTER TABLE translation ADD COLUMN updated_completion TEXT NOT NULL DEFAULT '';
