CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE kideaccounts (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    jwt TEXT NOT NULL
)
