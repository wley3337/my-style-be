-- Your SQL goes here
CREATE TABLE tags_clothing_items(
    id SERIAL,
    tag_id int REFERENCES tags (id) ON UPDATE CASCADE ON DELETE CASCADE,
    clothing_item_id int REFERENCES clothing_items (id) ON UPDATE CASCADE ON DELETE CASCADE,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    CONSTRAINT tag_clothing_item PRIMARY KEY(tag_id, clothing_item_id)
)
