-- Your SQL goes here
CREATE TABLE outfits(
    id SERIAL PRIMARY KEY,
    name varchar(255) NOT NULL,
    occasion varchar(255) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
)
