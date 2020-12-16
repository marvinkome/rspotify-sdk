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

pub async fn handle_fetch_playlist(id: &str, with_features: bool, client: &RSpotify) -> Result<()> {
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
