use polodb_core::{bson::{doc, to_bson}, CollectionT, Database};
use serde::{Deserialize, Serialize};
use crate::models::Song;
// use crate::prelude::AppState;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AppData {
    pub uuid: String,
    pub user_name: String,
    pub music_era: String,
    pub music_genre: String,
    
    pub tracks: Vec<Song>
}


pub fn get_playlist(db: &Database) -> Result<Vec<Song>, anyhow::Error> {
    let app_collection = db.collection::<AppData>("app_data");
    let result = app_collection.find_one(doc! {
        "uuid": "app_state_001"
    })?;

    match result {
        Some(data) => Ok(data.tracks),
        None => Ok(vec![])
    }
}

pub fn save_playlist(db: &Database, data: &Vec<Song>) -> Result<bool, anyhow::Error> {
    let app_collection = db.collection::<AppData>("app_data");
    
    let result = app_collection.update_one(doc! {
        "uuid": "app_state_001"
    }, doc! {
        "$set": doc! {
            "tracks": to_bson(&data)?,
        }
    })?;

    match result.modified_count > 0 {
        true => Ok(true),
        false => Ok(false)
    }
}

pub fn get_app_settings(db: &Database) -> anyhow::Result<AppData> {
    let app_collection = db.collection::<AppData>("app_data");
    let result = app_collection.find_one(doc! {
        "uuid": "app_state_001"
    })?;

    match result {
        Some(data) => Ok(data),
        None => {
            let data = AppData {
                uuid: "app_state_001".to_string(),
                user_name: "tor".to_string(),
                music_era: "Any".to_string(),
                music_genre: "Any".to_string(),
                tracks: vec![]
            };
            
            create_app_settings(db, &data)
                .expect("failed to save app settings");

            Ok(data)
        }
    }
}

pub fn create_app_settings(db: &Database, data: &AppData) -> Result<(), anyhow::Error> {
    let app_collection = db.collection::<AppData>("app_data");
    
    app_collection.insert_one(AppData {
        uuid: "app_state_001".to_string(),
        user_name: data.user_name.clone(),
        music_era: data.music_era.clone(),
        music_genre: data.music_genre.clone(),        
        tracks: vec![]
    })?;

    Ok(())
}

pub fn save_app_settings(db: &Database, user_name: String, music_era: String, music_genre: String) -> Result<bool, anyhow::Error> {
    let app_collection = db.collection::<AppData>("app_data");
    
    let result = app_collection.update_one(doc! {
        "uuid": "app_state_001"
    }, doc! {
        "$set": doc! {
            "user_name": user_name,
            "music_era": music_era,
            "music_genre": music_genre,
        }
    })?;
    
    match result.modified_count > 0 {
        true => Ok(true),
        false => Ok(false)
    }
}
