CREATE TABLE webhooks (
    id          SERIAL PRIMARY KEY,
    user_id     INTEGER NOT NULL UNIQUE,
    url         VARCHAR NOT NULL,
    created_at  TIMESTAMP NOT NULL DEFAULT now()
);