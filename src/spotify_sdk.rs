use crate::requests;
use crate::response::authorization::ClientAuthorizeResponse;
use crate::response::playlist::PlaylistTrack;
use crate::response::spotify_types::Track;
use base64::encode;
use log::info;
use reqwest::header;

const SPOTIFY_AUTH_URL: &'static str = "https://accounts.spotify.com/api/token";

pub struct Spotify {
    client_id: &'static str,
    client_secret: &'static str,
    token: Option<String>,
}

impl Spotify {
    pub async fn new(client_id: &'static str, client_secret: &'static str) -> Self {
        let mut spotify = Spotify {
            client_id,
            client_secret,
            token: None,
        };

        spotify.authorize().await;

        return spotify;
    }

    async fn authorize(&mut self) {
        let auth_key = format!("{}:{}", self.client_id, self.client_secret);
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
            Err(_error) => panic!("Error making request"),
        };

        let data = match resp.json::<ClientAuthorizeResponse>().await {
            Ok(resp) => resp,
            Err(_error) => panic!("Error getting data from response"),
        };

        self.token = Some(data.access_token);
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
}
