use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct PlaylistTrack {
    pub added_at: Option<String>,
    pub is_local: bool,
    pub track: super::spotify_types::Track,
}

pub type PlaylistTracksResponse = super::CollectionResponse<PlaylistTrack>;
