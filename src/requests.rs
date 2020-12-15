use crate::helpers;
use crate::response::playlist::PlaylistTracksResponse;
use crate::response::search::SearchTrackResponse;
use log::{info, warn};
use reqwest::Error;

const SPOTIFY_URL: &'static str = "https://api.spotify.com/v1";

pub async fn make_search_request(title: &str, artist: &str, token: &str) -> SearchTrackResponse {
    info!("Making search request for {} by {}", title, artist);

    let url = format!(
        "{}/search?type=track&q=track:{} artist:{}",
        SPOTIFY_URL, title, artist
    );

    let (client, headers) = helpers::generate_request(token);

    let resp = match client.get(&url).headers(headers).send().await {
        Ok(resp) => resp,
        Err(err) => {
            eprintln!("Something went wrong: Status: {:?}", err.status());
            std::process::exit(1);
        }
    };

    if resp.status().as_u16() > 299 {
        warn!("Something went wrong. Status: {:?}", resp.status());
        println!("Body:\n{}", resp.text().await.unwrap());
        std::process::exit(1);
    }

    info!("Data gotten from API");
    resp.json::<SearchTrackResponse>().await.unwrap()
}

pub async fn make_playlist_request(
    playlist_id: &str,
    token: &str,
    link: Option<&String>,
) -> Result<PlaylistTracksResponse, Error> {
    info!("Fetching playlist {}", playlist_id);
    let url = match link {
        Some(link) => link.to_owned(),
        None => format!("{}/playlists/{}/tracks", SPOTIFY_URL, playlist_id),
    };

    let (client, headers) = helpers::generate_request(token);

    let resp = match client.get(&url).headers(headers).send().await {
        Ok(resp) => resp,
        Err(err) => {
            eprintln!("Something went wrong: Status: {:?}", err.status());
            std::process::exit(1);
        }
    };

    if resp.status().as_u16() > 299 {
        warn!("Something went wrong. Status: {:?}", resp.status());
        println!("Body:\n{}", resp.text().await.unwrap());
        std::process::exit(1);
    }

    info!("Data gotten from API");
    let data = resp.json::<PlaylistTracksResponse>().await?;

    return Ok(data);
}
