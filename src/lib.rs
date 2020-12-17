pub mod response;
mod utils;

use crate::response::audio_features::AudioFeatures;
use crate::response::authorization::{ClientAuthorizeResponse, UserAuthorizeResponse};
use crate::response::playlist::PlaylistTrack;
use crate::response::spotify_types::Track;
use base64::encode;
use log::{info, warn};
use reqwest::header;
use utils::requests;

const SPOTIFY_AUTH_URL: &'static str = "https://accounts.spotify.com/api/token";

pub struct RSpotify {
    client_id: String,
    client_secret: String,
    token: Option<String>,
}

impl RSpotify {
    pub async fn new(
        client_id: String,
        client_secret: String,
        auth_type: Option<&str>,
        scope: Option<&str>,
    ) -> Self {
        let mut spotify = RSpotify {
            client_id,
            client_secret,
            token: None,
        };

        match auth_type {
            Some("user") => spotify.authorize_user(scope.unwrap()).await,
            _ => spotify.authorize().await,
        }

        return spotify;
    }

    async fn authorize(&mut self) {
        info!("Begin authorization");
        let auth_key = format!("{}:{}", &self.client_id, &self.client_secret);
        let auth_key = encode(auth_key.as_bytes());

        let client = reqwest::Client::new();

        let url = format!("{}?grant_type=client_credentials", SPOTIFY_AUTH_URL);
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::CONTENT_TYPE,
            header::HeaderValue::from_str("application/x-www-form-urlencoded").unwrap(),
        );
        headers.insert(
            header::AUTHORIZATION,
            header::HeaderValue::from_str(&format!("Basic {}", auth_key)).unwrap(),
        );
        headers.insert(
            header::CONTENT_LENGTH,
            header::HeaderValue::from_str("0").unwrap(),
        );

        let resp = match client.post(&url).headers(headers).send().await {
            Ok(resp) => resp,
            Err(error) => panic!("Error making auth request - {}", error),
        };

        if resp.status().as_u16() > 299 {
            warn!("Something went wrong. Status: {:?}", resp.status());
            println!("Body:\n{}", resp.text().await.unwrap());
            std::process::exit(1);
        }
        info!("Authorization completed");
        let data = resp.json::<ClientAuthorizeResponse>().await.unwrap();

        self.token = Some(data.access_token);
    }

    async fn authorize_user(&mut self, scope: &str) {
        info!("Begin user authorization");

        let auth_key = format!("{}:{}", &self.client_id, &self.client_secret);
        let auth_key = encode(auth_key.as_bytes());
        let refresh_token = utils::read_from_auth_cache("refresh_token");
        let mut code: String = String::new();

        // ask for code is no refresh token
        if refresh_token.is_err() {
            code = utils::open_browser_for_auth(&self.client_id, scope, false).unwrap();
        }

        // make authorization request
        let client = reqwest::Client::new();
        let url = match refresh_token {
            Ok(token) => format!(
                "{}?grant_type=refresh_token&refresh_token={}&redirect_ur\
            i=http://localhost:8008/callback",
                SPOTIFY_AUTH_URL, token
            ),
            Err(_e) => format!(
                "{}?grant_type=authorization_code&code={}&redirect_ur\
            i=http://localhost:8008/callback",
                SPOTIFY_AUTH_URL, code
            ),
        };

        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::CONTENT_TYPE,
            header::HeaderValue::from_str("application/x-www-form-urlencoded").unwrap(),
        );
        headers.insert(
            header::AUTHORIZATION,
            header::HeaderValue::from_str(&format!("Basic {}", auth_key)).unwrap(),
        );
        headers.insert(
            header::CONTENT_LENGTH,
            header::HeaderValue::from_str("0").unwrap(),
        );

        let resp = match client.post(&url).headers(headers).send().await {
            Ok(resp) => resp,
            Err(error) => panic!("Error making auth request - {}", error),
        };

        if resp.status().as_u16() > 299 {
            warn!("Something went wrong. Status: {:?}", resp.status());
            println!("Body:\n{}", resp.text().await.unwrap());
            std::process::exit(1);
        }

        // authorization completed
        info!("Authorization completed");
        let data = resp.json::<UserAuthorizeResponse>().await.unwrap();
        self.token = Some(data.access_token);

        if data.refresh_token.is_some() {
            let token = data.refresh_token.unwrap();
            match utils::write_to_auth_cache("refresh_token", &token) {
                Ok(_) => (),
                Err(e) => {
                    warn!("Failed to cache refresh token: {:?}", e);
                }
            }
        }
    }

    pub async fn search_track(&self, title: &str, artist: &str) -> Option<Track> {
        let data =
            requests::make_search_request(title, artist, &self.token.as_ref().unwrap()).await;

        data.tracks.items.into_iter().nth(0)
    }

    pub async fn get_playlist_tracks(&self, id: &str) -> Vec<PlaylistTrack> {
        let data = requests::make_playlist_request(id, &self.token.as_ref().unwrap(), None)
            .await
            .unwrap();
        let mut next = data.next;

        info!(
            "Setting initial items with current playlist - Length {:?}",
            data.items.len()
        );
        let mut songs = data.items;

        while next.is_some() {
            info!("Fetching next set of songs - {:?}", next);
            let data =
                requests::make_playlist_request(id, &self.token.as_ref().unwrap(), next.as_ref())
                    .await
                    .unwrap();

            next = data.next;

            let mut items = data.items;
            info!(
                "Merging items with current playlist - Length {:?}",
                items.len()
            );
            songs.append(&mut items);
        }

        return songs;
    }

    pub async fn get_album_tracks(&self, id: &str) -> Vec<Track> {
        let data = requests::make_album_request(id, &self.token.as_ref().unwrap(), None)
            .await
            .unwrap();
        let mut next = data.next;

        info!(
            "Setting initial items with current album - Length {:?}",
            data.items.len()
        );
        let mut songs = data.items;

        while next.is_some() {
            info!("Fetching next set of songs - {:?}", next);
            let data =
                requests::make_album_request(id, &self.token.as_ref().unwrap(), next.as_ref())
                    .await
                    .unwrap();

            next = data.next;

            let mut items = data.items;
            info!(
                "Merging items with current album - Length {:?}",
                items.len()
            );
            songs.append(&mut items);
        }

        return songs;
    }

    pub async fn get_audio_features(&self, track_ids: Vec<String>) -> Vec<AudioFeatures> {
        let mut audio_features: Vec<AudioFeatures> = Vec::new();

        let track_chunks = track_ids.chunks(100);

        for chunk in track_chunks {
            let data =
                requests::make_audio_features_request(chunk, &self.token.as_ref().unwrap()).await;
            let mut items = data.unwrap().audio_features;
            audio_features.append(&mut items);
        }

        return audio_features;
    }
}
