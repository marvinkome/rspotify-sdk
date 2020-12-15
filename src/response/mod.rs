use serde::Deserialize;

pub mod album;
pub mod authorization;
pub mod playlist;
pub mod search;
pub mod spotify_types;

#[derive(Deserialize, Debug)]
pub struct CollectionResponse<T> {
    pub href: String,
    pub items: Vec<T>,
    pub limit: u32,
    pub next: Option<String>,
    pub previous: Option<String>,
    pub offset: u32,
    pub total: u32,
}
