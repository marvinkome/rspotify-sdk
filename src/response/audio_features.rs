use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct AudioFeatures {
    pub danceability: f64,
    pub energy: f64,
    pub key: u64,
    pub loudness: f64,
    pub mode: f64,
    pub speechiness: f64,
    pub acousticness: f64,
    pub instrumentalness: f64,
    pub liveness: f64,
    pub valence: f64,
    pub tempo: f64,
    pub r#type: String,
    pub id: String,
    pub uri: String,
    pub track_href: String,
    pub analysis_url: String,
    pub duration_ms: u64,
    pub time_signature: u64,
}

#[derive(Deserialize, Debug)]
pub struct AudioFeaturesResponse {
    pub audio_features: Vec<AudioFeatures>,
}
