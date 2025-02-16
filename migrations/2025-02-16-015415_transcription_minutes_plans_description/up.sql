-- Your SQL goes here
ALTER TABLE languageai_subscription_plans ADD COLUMN description VARCHAR;
ALTER TABLE languageai_subscription_plans ADD COLUMN category VARCHAR DEFAULT 'generic';
ALTER TABLE speech_to_text ADD COLUMN audio_minutes INTEGER DEFAULT 1;