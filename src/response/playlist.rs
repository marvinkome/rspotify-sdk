use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct PlaylistTrack {
    pub added_at: Option<String>,
    pub is_local: bool,
    pub track: super::spotify_types::Track,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UserPlaylist {
    pub collaborative: bool,
    pub description: Option<String>,
    pub href: String,
    pub id: String,
    pub name: String,
    pub public: bool,
    pub snapshot_id: String,
    pub uri: String,
}

pub type PlaylistTracksResponse = super::CollectionResponse<PlaylistTrack>;
pub type UserPlaylistResponse = super::CollectionResponse<UserPlaylist>;
