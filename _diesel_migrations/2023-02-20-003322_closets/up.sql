-- Your SQL goes here
CREATE TABLE Closets(
    id SERIAL PRIMARY KEY,
    name varchar(255) NOT NULL,
    user_id int REFERENCES users (id) ON UPDATE CASCADE ON DELETE CASCADE,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
)
