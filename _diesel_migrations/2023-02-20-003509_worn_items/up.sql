-- Your SQL goes here
CREATE TABLE Worn_items(
    id SERIAL PRIMARY KEY,
    outfit_id int REFERENCES outfits (id) ON UPDATE CASCADE ON DELETE CASCADE,
    date_warn TIMESTAMP NOT NULL DEFAULT NOW(),
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
)
