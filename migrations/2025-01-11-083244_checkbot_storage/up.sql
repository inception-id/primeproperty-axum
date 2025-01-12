-- Your SQL goes here
CREATE TABLE checkbot_storage (
                          id SERIAL PRIMARY KEY NOT NULL,
                          user_id uuid NOT NULL REFERENCES users(id),
                        checkbot_id INT NOT NULL REFERENCES checkbot(id),
                          created_at TIMESTAMP NOT NULL DEFAULT NOW(),
                          updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
                          instruction VARCHAR NOT NULL,
                          content TEXT NOT NULL,
                          updated_completion TEXT NOT NULL
);
SELECT diesel_manage_updated_at('checkbot_storage');
