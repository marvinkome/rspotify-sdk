use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct PlaylistTrack {
    pub added_at: Option<String>,
    pub is_local: bool,
    pub track: super::spotify_types::Track,
}

#[derive(Deserialize, Debug)]
pub struct PlaylistTracksResponse {
    pub href: String,
    pub items: Vec<PlaylistTrack>,
    pub limit: u32,
    pub next: Option<String>,
    pub previous: Option<String>,
    pub offset: u32,
    pub total: u32,
}
