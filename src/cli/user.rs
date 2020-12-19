use rspotify_sdk::response::spotify_types::Track;
use rspotify_sdk::RSpotify;

pub struct UserData {
    pub songs: Vec<Track>,
    client: RSpotify,
}

impl UserData {
    pub async fn new(client_id: &str, client_secret: &str) -> Self {
        let client = RSpotify::new(
            client_id.to_owned(),
            client_secret.to_owned(),
            Some("user"),
            Some("playlist-read-private user-library-read"),
        )
        .await;

        let user_data = UserData {
            songs: Vec::new(),
            client,
        };

        return user_data;
    }

    pub async fn get_playlists_track(&mut self) {
        // get all playlist
        let playlists = self.client.get_user_playlists().await;

        for playlist in playlists {
            // get track in playlist
            let tracks = self.client.get_playlist_tracks(&playlist.id).await;
            let mut tracks: Vec<Track> = tracks
                .into_iter()
                .map(|playlist_track| playlist_track.track.clone())
                .collect();

            self.songs.append(&mut tracks);
        }
    }

    pub async fn get_albums_track(&mut self) {
        // get all playlist
        let albums = self.client.get_user_albums().await;

        for album in albums {
            // get track in album
            let tracks = self.client.get_album_tracks(&album.album.id).await;
            let mut tracks: Vec<Track> = tracks.into_iter().map(|track| track.clone()).collect();

            self.songs.append(&mut tracks);
        }
    }

    pub async fn get_liked_songs(&mut self) {
        // get all playlist
        let saved_tracks = self.client.get_user_liked_songs().await;

        // get track in playlist
        let mut tracks: Vec<Track> = saved_tracks
            .into_iter()
            .map(|saved_track| saved_track.track.clone())
            .collect();

        self.songs.append(&mut tracks);
    }
}
