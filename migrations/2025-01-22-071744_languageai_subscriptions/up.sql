-- Your SQL goes here
CREATE TABLE languageai_subscriptions (
                                                  id SERIAL PRIMARY KEY NOT NULL,
                                                  user_id uuid NOT NULL REFERENCES users(id),
                                                  languageai_subscription_plan_id INTEGER NOT NULL REFERENCES languageai_subscription_plans(id),
                                                  languageai_subscription_payment_id INTEGER NOT NULL REFERENCES languageai_subscription_payments(id),
                                                  created_at TIMESTAMP NOT NULL DEFAULT NOW(),
                                                  updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
                                                  expired_at TIMESTAMP NOT NULL,
                                                    history_limit INTEGER,
                                                  storage_limit INTEGER,
                                                  translation_limit INTEGER,
                                                  checkbot_limit INTEGER,
                                                  text_to_speech_limit INTEGER,
                                                  speech_to_text_limit INTEGER
);
SELECT diesel_manage_updated_at('languageai_subscriptions');
