// @generated automatically by Diesel CLI.

diesel::table! {
    app_data (uuid) {
        uuid -> Text,
        user_name -> Text,
        music_era -> Text,
        music_genre -> Text,
        provider -> Text,
    }
}

diesel::table! {
    tracks (uuid) {
        uuid -> Text,
        app_data_id -> Text,
        title -> Text,
        artist -> Text,
        album -> Text,
        cover_art -> Text,
        preview_url -> Text,
    }
}

diesel::joinable!(tracks -> app_data (app_data_id));

diesel::allow_tables_to_appear_in_same_query!(
    app_data,
    tracks,
);
