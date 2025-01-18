-- Your SQL goes here
CREATE TABLE languageai_subscription_plans (
                           id SERIAL PRIMARY KEY NOT NULL,
                           created_at TIMESTAMP NOT NULL DEFAULT NOW(),
                           updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
                           name VARCHAR NOT NULL,
                            initial_price DECIMAL NOT NULL,
                            discounted_price DECIMAL,
                            history_limit  INTEGER,
                            storage_limit INTEGER,
                            translation_limit INTEGER,
                            checkbot_limit INTEGER,
                            text_to_speech_limit INTEGER,
                            speech_to_text_limit INTEGER
);
SELECT diesel_manage_updated_at('languageai_subscription_plans');
