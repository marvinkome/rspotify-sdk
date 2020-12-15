use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct AlbumCover {
    pub height: u32,
    pub url: String,
    pub width: u32,
}

#[derive(Deserialize, Debug)]
pub struct Album {
    pub album_type: String,
    pub available_markets: Vec<String>,
    pub href: String,
    pub id: String,
    pub images: Vec<AlbumCover>,
    pub name: String,
    pub release_date: String,
    pub release_date_precision: String,
    pub total_tracks: u32,
    pub r#type: String,
    pub uri: String,
}

#[derive(Deserialize, Debug)]
pub struct Artist {
    pub href: String,
    pub id: String,
    pub name: String,
    pub r#type: String,
    pub uri: String,
}

#[derive(Deserialize, Debug)]
pub struct Track {
    pub album: Album,
    pub artists: Vec<Artist>,
    pub available_markets: Vec<String>,
    pub disc_number: u32,
    pub duration_ms: u32,
    pub explicit: bool,
    pub href: String,
    pub id: String,
    pub name: String,
    pub popularity: u32,
    pub preview_url: Option<String>,
    pub track_number: u32,
    pub r#type: String,
    pub uri: String,
}
