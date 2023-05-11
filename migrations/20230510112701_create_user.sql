-- Add migration script here
CREATE TABLE users (
	id SERIAL PRIMARY KEY,
	first_name TEXT NOT NULL,
	last_name TEXT NOT NULL
);
