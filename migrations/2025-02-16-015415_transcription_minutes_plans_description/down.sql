-- This file should undo anything in `up.sql`
ALTER TABLE languageai_subscription_plans DROP COLUMN description;
ALTER TABLE languageai_subscription_plans DROP COLUMN category;
ALTER TABLE languageai_subscription_plans DROP COLUMN is_active;
ALTER TABLE speech_to_text DROP COLUMN audio_minutes;
