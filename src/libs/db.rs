use diesel::prelude::*;
use dioxus::logger::tracing::info;

use super::schema::app_data::columns::uuid;
use super::schema::{app_data::dsl::*, tracks::dsl::*};
use crate::models::Song;
use crate::pages::settings::AppSettings;

#[derive(Identifiable, Insertable, Queryable, Selectable, Clone, Debug)]
#[diesel(table_name = super::schema::app_data)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[primary_key(uuid)]
pub struct AppData {
    pub uuid: String,
    pub user_name: String,
    pub music_era: String,
    pub music_genre: String,
    pub provider: String,
}

#[derive(Identifiable, Insertable, Queryable, Selectable, Associations, Debug, PartialEq)]
#[diesel(belongs_to(AppData))]
#[diesel(table_name = super::schema::tracks)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[primary_key(uuid)]
pub struct Track {
    pub uuid: String,
    pub app_data_id: String,
    pub title: String,
    pub artist: String,
    pub album: String,
    pub cover_art: String,
    pub preview_url: String,
}

impl From<Track> for Song {
    fn from(value: Track) -> Self {
        Song {
            uuid: value.uuid,
            title: value.title,
            artist: value.artist,
            album: value.album,
            cover_art: value.cover_art,
            preview_url: value.preview_url,
        }
    }
}

pub fn create_db() -> SqliteConnection {
    //let database_url = env!("DATABASE_URL");
    SqliteConnection::establish(&"/Users/tor/Documents/projects/st.faus/db/stfaus.db")
        .unwrap_or_else(|e| panic!("Error: {:?}", e.to_string()))
}

pub fn get_playlist(db_connection: &mut SqliteConnection) -> Result<Vec<Song>, String> {
    let data = app_data
        .filter(uuid.eq("app_state_001"))
        .select(AppData::as_select())
        .first::<AppData>(db_connection)
        .map_err(|e| e.to_string())?;

    let last_playlist: Vec<Track> = Track::belonging_to(&data)
        .load(db_connection)
        .map_err(|e| e.to_string())?;

    Ok(last_playlist.into_iter().map(Song::from).collect())
}

pub fn save_playlist(db_connection: &mut SqliteConnection, data: &Vec<Song>) -> Result<(), String> {
    diesel::delete(tracks)
        .execute(db_connection)
        .expect("failed to delete tracks records");

    let app_settings = get_app_settings(db_connection);
    let data = data
        .iter()
        .map(|t| t.into_track(&app_settings.uuid))
        .collect::<Vec<Track>>();

    diesel::insert_into(tracks)
        .values(&data)
        .execute(db_connection)
        .map_err(|e| e.to_string())?;

    Ok(())
}

pub fn get_app_settings(db_connection: &mut SqliteConnection) -> AppData {
    let app_settings = app_data
        .filter(uuid.eq("app_state_001")) // move id to config.toml env
        .select(AppData::as_select())
        .first::<AppData>(db_connection);

    let result = match app_settings {
        Ok(d) => d,
        Err(_) => {
            let d = AppData {
                uuid: "app_state_001".to_string(),
                user_name: "tor".to_string(),
                music_era: "Any".to_string(),
                music_genre: "Any".to_string(),
                provider: "Spotify".to_string(),
            };

            create_app_settings(
                db_connection,
                &AppSettings {
                    account_name: d.user_name.clone(),
                    music_era: d.music_era.clone(),
                    music_genre: d.music_genre.clone(),
                    provider: d.provider.clone(),
                },
            )
            .expect("failed to save app settings");

            d
        }
    };

    result
}

pub fn create_app_settings(
    db_connection: &mut SqliteConnection,
    data: &AppSettings,
) -> Result<(), String> {
    diesel::insert_into(app_data)
        .values(AppData {
            uuid: "app_state_001".to_string(),
            user_name: data.account_name.clone(),
            music_era: data.music_era.clone(),
            music_genre: data.music_genre.clone(),
            provider: data.provider.clone(),
        })
        .execute(db_connection)
        .map_err(|e| e.to_string())?;

    Ok(())
}

pub fn save_app_settings(db_connection: &mut SqliteConnection, data: &AppSettings) -> Result<(), String>{
    diesel::update(app_data)
        .set((
            user_name.eq(data.account_name.clone()),
            music_era.eq(data.music_era.clone()),
            music_genre.eq(data.music_genre.clone()),
            provider.eq(data.provider.clone())
        ))
        .execute(db_connection)
        .map_err(|e| e.to_string())?;

    Ok(())
}
