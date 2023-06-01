-- Your SQL goes here
CREATE TABLE Clothing_items(
    id SERIAL PRIMARY KEY,
    closet_id int REFERENCES closets (id),
    name varchar(255) NOT NULL,
    image_url varchar(255) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
)
