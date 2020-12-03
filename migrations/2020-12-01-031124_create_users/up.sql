-- Your SQL goes here
CREATE TABLE users (
  id serial PRIMARY KEY,
  username VARCHAR(127) NOT NULL UNIQUE,
  password VARCHAR(127) NOT NULL
)