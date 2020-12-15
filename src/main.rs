use serde::Deserialize;

mod helpers;
mod requests;
mod response;
mod spotify_sdk;

#[derive(Deserialize, Debug)]
struct Config {
    client_id: String,
    client_secret: String,
}

#[tokio::main]
async fn main() {
    // SETUP
    dotenv::dotenv().expect("Can't load end. Take down program");
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();
    let env = envy::from_env::<Config>().unwrap();

    // load lib
    let spotify = spotify_sdk::Spotify::new(env.client_id, env.client_secret).await;

    // let _searched_song = spotify.search_track("Morph", "Twenty one pilots").await;
    // println!("{:?}", _searched_song.unwrap());

    // let _playlist_tracks = spotify.get_playlist_tracks("6f3lchHmBQed8GnWmayLn6").await;
    // println!("{:?}", _playlist_tracks.len());

    // let _album_tracks = spotify.get_album_tracks("5oT2zoIrVGJcbVZoNGGZwc").await;
    // println!("{:?}", _album_tracks.len());

    let _audio_features = spotify
        .get_audio_features(vec![
            "3UaFnnUo80mv431WHEzaj9".to_string(),
            "77vDHmiANW3JS2gNN5q7pI".to_string(),
            "6QOZWUCG5vq4xtU3mW2ZA2".to_string(),
            "4bk6v5SBxNoVsbOvdOvUAJ".to_string(),
        ])
        .await;
    println!("{:?}", _audio_features);
}
