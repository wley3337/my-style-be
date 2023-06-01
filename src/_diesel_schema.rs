// @generated automatically by Diesel CLI.

diesel::table! {
    closets (id) {
        id -> Int4,
        name -> Varchar,
        user_id -> Nullable<Int4>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    clothing_items (id) {
        id -> Int4,
        closet_id -> Nullable<Int4>,
        name -> Varchar,
        image_url -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    outfits (id) {
        id -> Int4,
        name -> Varchar,
        occasion -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    outfits_clothing_items (outfit_id, clothing_item_id) {
        id -> Int4,
        outfit_id -> Int4,
        clothing_item_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    outfits_seasons (outfit_id, season_id) {
        id -> Int4,
        outfit_id -> Int4,
        season_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    outfits_users (outfit_id, user_id) {
        id -> Int4,
        outfit_id -> Int4,
        user_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    seasons (id) {
        id -> Int4,
        name -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    tags (id) {
        id -> Int4,
        name -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    tags_clothing_items (tag_id, clothing_item_id) {
        id -> Int4,
        tag_id -> Int4,
        clothing_item_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    tags_users_items (tag_id, user_id) {
        id -> Int4,
        tag_id -> Int4,
        user_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        password_hash -> Varchar,
        display_name -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    worn_items (id) {
        id -> Int4,
        outfit_id -> Nullable<Int4>,
        date_warn -> Timestamp,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(closets -> users (user_id));
diesel::joinable!(clothing_items -> closets (closet_id));
diesel::joinable!(outfits_clothing_items -> clothing_items (clothing_item_id));
diesel::joinable!(outfits_clothing_items -> outfits (outfit_id));
diesel::joinable!(outfits_seasons -> outfits (outfit_id));
diesel::joinable!(outfits_seasons -> seasons (season_id));
diesel::joinable!(outfits_users -> outfits (outfit_id));
diesel::joinable!(outfits_users -> users (user_id));
diesel::joinable!(tags_clothing_items -> clothing_items (clothing_item_id));
diesel::joinable!(tags_clothing_items -> tags (tag_id));
diesel::joinable!(tags_users_items -> tags (tag_id));
diesel::joinable!(tags_users_items -> users (user_id));
diesel::joinable!(worn_items -> outfits (outfit_id));

diesel::allow_tables_to_appear_in_same_query!(
    closets,
    clothing_items,
    outfits,
    outfits_clothing_items,
    outfits_seasons,
    outfits_users,
    seasons,
    tags,
    tags_clothing_items,
    tags_users_items,
    users,
    worn_items,
);
