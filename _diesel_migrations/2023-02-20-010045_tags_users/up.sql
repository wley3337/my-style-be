-- Your SQL goes here
CREATE TABLE tags_users_items(
    id SERIAL,
    tag_id int REFERENCES tags (id) ON UPDATE CASCADE ON DELETE CASCADE,
    user_id int REFERENCES users (id) ON UPDATE CASCADE ON DELETE CASCADE,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    CONSTRAINT tag_user PRIMARY KEY(tag_id, user_id)
)
