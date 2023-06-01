-- Your SQL goes here
CREATE TABLE outfits_seasons(
    id SERIAL,
    outfit_id int REFERENCES outfits (id) ON UPDATE CASCADE ON DELETE CASCADE,
    season_id int REFERENCES seasons (id) ON UPDATE CASCADE ON DELETE CASCADE,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    CONSTRAINT outfit_season PRIMARY KEY(outfit_id, season_id)
)
