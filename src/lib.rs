pub mod response;
mod utils;

use crate::response::album::UserAlbum;
use crate::response::audio_features::AudioFeatures;
use crate::response::authorization::{ClientAuthorizeResponse, UserAuthorizeResponse};
use crate::response::playlist::{PlaylistTrack, UserPlaylist};
use crate::response::spotify_types::Track;
use crate::response::track::SavedTrack;
use base64::encode;
use log::{info, warn};
use reqwest::header;
use utils::requests;

const SPOTIFY_AUTH_URL: &'static str = "https://accounts.spotify.com/api/token";

pub struct RSpotify {
    client_id: String,
    client_secret: String,
    token: Option<String>,
    request: Option<requests::SpotifyRequest>,
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
            request: None,
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

        self.request = Some(requests::SpotifyRequest::new(&data.access_token));
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

        self.request = Some(requests::SpotifyRequest::new(&data.access_token));
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
        let request = self.request.as_ref().unwrap();
        let data = request.make_search_request(title, artist).await.unwrap();
        data.tracks.items.into_iter().nth(0)
    }

    pub async fn get_playlist_tracks(&self, id: &str) -> Vec<PlaylistTrack> {
        let request = self.request.as_ref().unwrap();
        let data = request.make_playlist_request(id, None).await.unwrap();

        let mut next = data.next;

        let mut songs = data.items;

        while next.is_some() {
            let data = request
                .make_playlist_request(id, next.as_ref())
                .await
                .unwrap();

            next = data.next;

            let mut items = data.items;
            songs.append(&mut items);
        }

        return songs;
    }

    pub async fn get_album_tracks(&self, id: &str) -> Vec<Track> {
        let request = self.request.as_ref().unwrap();
        let data = request.make_album_request(id, None).await.unwrap();
        let mut next = data.next;

        let mut songs = data.items;

        while next.is_some() {
            let data = request.make_album_request(id, next.as_ref()).await.unwrap();

            next = data.next;

            let mut items = data.items;
            songs.append(&mut items);
        }

        return songs;
    }

    pub async fn get_audio_features(&self, track_ids: Vec<String>) -> Vec<AudioFeatures> {
        let request = self.request.as_ref().unwrap();
        let mut audio_features: Vec<AudioFeatures> = Vec::new();

        let track_chunks = track_ids.chunks(100);

        for chunk in track_chunks {
            let data = request.make_audio_features_request(chunk).await.unwrap();
            let mut items = data.audio_features;
            audio_features.append(&mut items);
        }

        return audio_features;
    }

    pub async fn get_user_playlists(&self) -> Vec<UserPlaylist> {
        let request = self.request.as_ref().unwrap();
        let data = request.make_user_playlist_request(None).await.unwrap();
        let mut next = data.next;

        let mut songs = data.items;

        while next.is_some() {
            let data = request
                .make_user_playlist_request(next.as_ref())
                .await
                .unwrap();

            next = data.next;

            let mut items = data.items;
            songs.append(&mut items);
        }

        return songs;
    }

    pub async fn get_user_albums(&self) -> Vec<UserAlbum> {
        let request = self.request.as_ref().unwrap();
        let data = request.make_user_album_request(None).await.unwrap();
        let mut next = data.next;

        let mut songs = data.items;

        while next.is_some() {
            let data = request
                .make_user_album_request(next.as_ref())
                .await
                .unwrap();

            next = data.next;

            let mut items = data.items;
            songs.append(&mut items);
        }

        return songs;
    }

    pub async fn get_user_liked_songs(&self) -> Vec<SavedTrack> {
        let request = self.request.as_ref().unwrap();
        let data = request.make_user_saved_song_request(None).await.unwrap();
        let mut next = data.next;

        let mut songs = data.items;

        while next.is_some() {
            let data = request
                .make_user_saved_song_request(next.as_ref())
                .await
                .unwrap();

            next = data.next;

            let mut items = data.items;
            songs.append(&mut items);
        }

        return songs;
    }
}
