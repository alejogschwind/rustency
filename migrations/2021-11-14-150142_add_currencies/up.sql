-- Your SQL goes here
CREATE TABLE currencies
(
    id SERIAL NOT NULL PRIMARY KEY,
    symbol VARCHAR NOT NULL,
    name VARCHAR NOT NULL,
    price INTEGER NOT NULL,
    decimal_point INTEGER NOT NULL,
    created_at TIMESTAMP NOT NULL
);