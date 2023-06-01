-- Your SQL goes here
CREATE TABLE outfits_users(
    id SERIAL,
    outfit_id int REFERENCES outfits (id) ON UPDATE CASCADE ON DELETE CASCADE,
    user_id int REFERENCES users (id) ON UPDATE CASCADE ON DELETE CASCADE,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    CONSTRAINT outfit_user PRIMARY KEY(outfit_id, user_id)
)
