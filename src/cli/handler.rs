use crate::Config;
use rspotify_sdk::response::audio_features::AudioFeatures;
use rspotify_sdk::response::spotify_types::Track;
use rspotify_sdk::RSpotify;
use serde::Serialize;
use serde_json::Result;
use std::io::Write;

#[derive(Serialize, Debug)]
struct TrackWithFeatures {
    #[serde(flatten)]
    track: Track,

    #[serde(flatten)]
    features: AudioFeatures,
}

pub async fn handle_fetch_playlist(id: &str, with_features: bool, env: Config) -> Result<()> {
    let client = RSpotify::new(env.client_id, env.client_secret, Some("user"), Some("")).await;
    let data = client.get_playlist_tracks(id).await;

    if with_features {
        let track_ids: Vec<String> = data
            .iter()
            .map(|playlist_track| playlist_track.track.id.clone())
            .collect();

        let features = client.get_audio_features(track_ids).await;

        // merge data with features
        let data_with_features: Vec<TrackWithFeatures> = data
            .iter()
            .zip(features)
            .map(|(playlist_track, features)| TrackWithFeatures {
                track: playlist_track.track.clone(),
                features: features,
            })
            .collect();

        let json_resp = serde_json::to_string(&data_with_features)?;
        std::io::stdout().write_all(json_resp.as_bytes()).unwrap();
        return Ok(());
    }

    let json_resp = serde_json::to_string(&data)?;
    std::io::stdout().write_all(json_resp.as_bytes()).unwrap();
    return Ok(());
}

pub async fn handle_fetch_album(id: &str, with_features: bool, env: Config) -> Result<()> {
    let client = RSpotify::new(env.client_id, env.client_secret, None, None).await;
    let data = client.get_album_tracks(id).await;

    if with_features {
        let track_ids: Vec<String> = data.iter().map(|track| track.id.clone()).collect();

        let features = client.get_audio_features(track_ids).await;

        // merge data with features
        let data_with_features: Vec<TrackWithFeatures> = data
            .iter()
            .zip(features)
            .map(|(track, features)| TrackWithFeatures {
                track: track.clone(),
                features: features,
            })
            .collect();

        let json_resp = serde_json::to_string(&data_with_features)?;
        std::io::stdout().write_all(json_resp.as_bytes()).unwrap();
        return Ok(());
    }

    let json_resp = serde_json::to_string(&data)?;
    std::io::stdout().write_all(json_resp.as_bytes()).unwrap();
    return Ok(());
}

pub async fn handle_search_song(
    title: &str,
    artist: &str,
    with_features: bool,
    env: Config,
) -> Result<()> {
    let client = RSpotify::new(env.client_id, env.client_secret, None, None).await;
    let data = match client.search_track(title, artist).await {
        Some(track) => track,
        None => panic!("Track not found"),
    };

    if with_features {
        let track_ids: Vec<String> = vec![data.id.clone()];

        let features = match client.get_audio_features(track_ids).await.into_iter().nth(0) {
            Some(feat) => feat,
            None => panic!("Can't get track features. To get the track without features run the command without the --with-features flag")
        };

        // merge data with features
        let data_with_features = TrackWithFeatures {
            track: data,
            features,
        };

        let json_resp = serde_json::to_string(&data_with_features)?;
        std::io::stdout().write_all(json_resp.as_bytes()).unwrap();
        return Ok(());
    }

    let json_resp = serde_json::to_string(&data)?;
    std::io::stdout().write_all(json_resp.as_bytes()).unwrap();
    return Ok(());
}

pub async fn handle_get_user_playlists(with_features: bool, env: Config) -> Result<()> {
    let mut user_data = super::user::UserData::new(&env.client_id, &env.client_secret).await;
    user_data.get_playlists_track().await;

    let tracks = user_data.songs;

    if with_features {
        let client = RSpotify::new(env.client_id, env.client_secret, None, None).await;
        let track_ids: Vec<String> = tracks.iter().map(|track| track.id.clone()).collect();

        let features = client.get_audio_features(track_ids).await;

        // merge data with features
        let data_with_features: Vec<TrackWithFeatures> = tracks
            .iter()
            .zip(features)
            .map(|(track, features)| TrackWithFeatures {
                track: track.clone(),
                features: features,
            })
            .collect();

        let json_resp = serde_json::to_string(&data_with_features)?;
        std::io::stdout().write_all(json_resp.as_bytes()).unwrap();
        return Ok(());
    }

    let json_resp = serde_json::to_string(&tracks)?;
    std::io::stdout().write_all(json_resp.as_bytes()).unwrap();
    return Ok(());
}

pub async fn handle_get_user_albums(with_features: bool, env: Config) -> Result<()> {
    let mut user_data = super::user::UserData::new(&env.client_id, &env.client_secret).await;
    user_data.get_albums_track().await;

    let tracks = user_data.songs;

    if with_features {
        let client = RSpotify::new(env.client_id, env.client_secret, None, None).await;
        let track_ids: Vec<String> = tracks.iter().map(|track| track.id.clone()).collect();

        let features = client.get_audio_features(track_ids).await;

        // merge data with features
        let data_with_features: Vec<TrackWithFeatures> = tracks
            .iter()
            .zip(features)
            .map(|(track, features)| TrackWithFeatures {
                track: track.clone(),
                features: features,
            })
            .collect();

        let json_resp = serde_json::to_string(&data_with_features)?;
        std::io::stdout().write_all(json_resp.as_bytes()).unwrap();
        return Ok(());
    }

    let json_resp = serde_json::to_string(&tracks)?;
    std::io::stdout().write_all(json_resp.as_bytes()).unwrap();
    return Ok(());
}

pub async fn handle_get_liked_songs(with_features: bool, env: Config) -> Result<()> {
    let mut user_data = super::user::UserData::new(&env.client_id, &env.client_secret).await;
    user_data.get_liked_songs().await;

    let tracks = user_data.songs;

    if with_features {
        let client = RSpotify::new(env.client_id, env.client_secret, None, None).await;
        let track_ids: Vec<String> = tracks.iter().map(|track| track.id.clone()).collect();

        let features = client.get_audio_features(track_ids).await;

        // merge data with features
        let data_with_features: Vec<TrackWithFeatures> = tracks
            .iter()
            .zip(features)
            .map(|(track, features)| TrackWithFeatures {
                track: track.clone(),
                features: features,
            })
            .collect();

        let json_resp = serde_json::to_string(&data_with_features)?;
        std::io::stdout().write_all(json_resp.as_bytes()).unwrap();
        return Ok(());
    }

    let json_resp = serde_json::to_string(&tracks)?;
    std::io::stdout().write_all(json_resp.as_bytes()).unwrap();
    return Ok(());
}
