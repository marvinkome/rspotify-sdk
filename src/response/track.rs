use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct SavedTrack {
    pub added_at: Option<String>,
    pub track: super::spotify_types::Track,
}

pub type SavedTrackResponse = super::CollectionResponse<SavedTrack>;
