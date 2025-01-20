-- Your SQL goes here
CREATE TYPE subscription_period AS ENUM ('one_year', 'three_months', 'one_month');
CREATE TYPE payment_status AS ENUM ('success', 'pending', 'fail');
CREATE TABLE languageai_subscription_payments (
                                            id SERIAL PRIMARY KEY NOT NULL,
                                            user_id uuid NOT NULL REFERENCES users(id),
                                            languageai_subscription_plan_id INTEGER NOT NULL REFERENCES languageai_subscription_plans(id),
                                            created_at TIMESTAMP NOT NULL DEFAULT NOW(),
                                            updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
                                            expired_at TIMESTAMP NOT NULL,
                                            amount DECIMAL NOT NULL,
                                            period subscription_period NOT NULL,
                                            status payment_status NOT NULL DEFAULT 'pending',
                                            doku_request JSONB,
                                            doku_response JSONB,
                                            doku_notification JSONB
);
SELECT diesel_manage_updated_at('languageai_subscription_payments');
