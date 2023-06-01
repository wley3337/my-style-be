-- Your SQL goes here
CREATE TABLE Users(
    id SERIAL PRIMARY KEY,
    username varchar(255) NOT NULL,
    password_hash varchar(255) NOT NULL,
    display_name varchar(255) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
)
