-- Your SQL goes here
CREATE TABLE outfits_clothing_items(
    id SERIAL,
    outfit_id int REFERENCES outfits (id) ON UPDATE CASCADE ON DELETE CASCADE,
    clothing_item_id int REFERENCES clothing_items (id) ON UPDATE CASCADE ON DELETE CASCADE,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    CONSTRAINT outfit_clothing_item PRIMARY KEY(outfit_id, clothing_item_id)
)
