mod helpers;
mod requests;
mod response;
mod spotify_sdk;

#[tokio::main]
async fn main() {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    let spotify = spotify_sdk::Spotify::new(
        "c8e126040a874c10b8a95721f2ee1e40",
        "e5acaf30069c462892689751881589d3",
    )
    .await;

    // let _searched_song = spotify.search_track("Morph", "Twenty one pilots").await;
    // println!("{:?}", _searched_song.unwrap());

    let _playlist_tracks = spotify.get_playlist_tracks("6f3lchHmBQed8GnWmayLn6").await;
    println!("{:?}", _playlist_tracks.len());
}
