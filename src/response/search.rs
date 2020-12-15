use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct TrackResponse {
    pub href: String,
    pub items: Vec<super::spotify_types::Track>,
}

#[derive(Deserialize, Debug)]
pub struct SearchTrackResponse {
    pub tracks: TrackResponse,
}
