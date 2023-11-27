CREATE TABLE IF NOT EXISTS numbers (
    id serial PRIMARY KEY,
    value int NOT NULL UNIQUE,
    is_prime boolean not null
);
