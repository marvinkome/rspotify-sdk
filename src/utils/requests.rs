use super::raxios::RaxiosClient;
use crate::response::album::{AlbumTracksResponse, UserAlbumResponse};
use crate::response::audio_features::AudioFeaturesResponse;
use crate::response::playlist::{PlaylistTracksResponse, UserPlaylistResponse};
use crate::response::search::SearchTrackResponse;
use crate::response::track::SavedTrackResponse;
use log::info;
use reqwest::Error;

const SPOTIFY_URL: &'static str = "https://api.spotify.com/v1";

pub struct SpotifyRequest {
    raxios: RaxiosClient,
}

impl SpotifyRequest {
    pub fn new(token: &str) -> Self {
        let mut raxios = RaxiosClient::new();
        raxios.set_token(token);

        SpotifyRequest { raxios }
    }

    pub async fn make_search_request(
        &self,
        title: &str,
        artist: &str,
    ) -> Result<SearchTrackResponse, Error> {
        info!("Making search request for {} by {}", title, artist);

        let url = format!(
            "{}/search?type=track&q=track:{} artist:{}",
            SPOTIFY_URL, title, artist
        );

        let data = self.raxios.get::<SearchTrackResponse>(&url, None).await?;
        Ok(data)
    }

    pub async fn make_playlist_request(
        &self,
        playlist_id: &str,
        link: Option<&String>,
    ) -> Result<PlaylistTracksResponse, Error> {
        info!("Fetching playlist {}", playlist_id);

        let url = match link {
            Some(link) => link.to_owned(),
            None => format!("{}/playlists/{}/tracks", SPOTIFY_URL, playlist_id),
        };

        let data = self
            .raxios
            .get::<PlaylistTracksResponse>(&url, None)
            .await?;
        Ok(data)
    }

    pub async fn make_album_request(
        &self,
        album_id: &str,
        link: Option<&String>,
    ) -> Result<AlbumTracksResponse, Error> {
        info!("Fetching album {}", album_id);

        let url = match link {
            Some(link) => link.to_owned(),
            None => format!("{}/albums/{}/tracks", SPOTIFY_URL, album_id),
        };

        let data = self.raxios.get::<AlbumTracksResponse>(&url, None).await?;
        Ok(data)
    }

    pub async fn make_audio_features_request(
        &self,
        track_ids: &[String],
    ) -> Result<AudioFeaturesResponse, Error> {
        info!("Fetching features for {} tracks", track_ids.len());

        //
        let ids = track_ids.join(",");
        let url = format!("{}/audio-features?ids={}", SPOTIFY_URL, ids);

        let data = self.raxios.get::<AudioFeaturesResponse>(&url, None).await?;
        Ok(data)
    }

    pub async fn make_user_playlist_request(
        &self,
        link: Option<&String>,
    ) -> Result<UserPlaylistResponse, Error> {
        info!("Fetching user playlists");

        let url = match link {
            Some(link) => link.to_owned(),
            None => format!("{}/me/playlists", SPOTIFY_URL),
        };

        let data = self.raxios.get::<UserPlaylistResponse>(&url, None).await?;
        Ok(data)
    }

    pub async fn make_user_album_request(
        &self,
        link: Option<&String>,
    ) -> Result<UserAlbumResponse, Error> {
        info!("Fetching user albums");

        let url = match link {
            Some(link) => link.to_owned(),
            None => format!("{}/me/albums", SPOTIFY_URL),
        };

        let data = self.raxios.get::<UserAlbumResponse>(&url, None).await?;
        Ok(data)
    }

    pub async fn make_user_saved_song_request(
        &self,
        link: Option<&String>,
    ) -> Result<SavedTrackResponse, Error> {
        info!("Fetching user saved songs");

        let url = match link {
            Some(link) => link.to_owned(),
            None => format!("{}/me/tracks", SPOTIFY_URL),
        };

        let data = self.raxios.get::<SavedTrackResponse>(&url, None).await?;
        Ok(data)
    }
}
