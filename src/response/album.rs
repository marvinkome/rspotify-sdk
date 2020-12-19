use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct UserAlbum {
    pub added_at: String,
    pub album: super::spotify_types::Album,
}

pub type AlbumTracksResponse = super::CollectionResponse<super::spotify_types::Track>;
pub type UserAlbumResponse = super::CollectionResponse<UserAlbum>;
