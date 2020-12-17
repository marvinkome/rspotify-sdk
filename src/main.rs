mod cli;

use cli::RSpotifyCli;
use serde::Deserialize;
use serde_json::Result;
use structopt::StructOpt;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub client_id: String,
    pub client_secret: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    // SETUP
    dotenv::dotenv().expect("Can't load end. Take down program");
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    // INIT
    let env = envy::from_env::<Config>().unwrap();
    let args: RSpotifyCli = RSpotifyCli::from_args();

    // Handle args
    match args {
        RSpotifyCli::GetPlaylistTracks { id, with_features } => {
            cli::handler::handle_fetch_playlist(&id, with_features, env).await?;
        }
        RSpotifyCli::GetAlbumTracks { id, with_features } => {
            cli::handler::handle_fetch_album(&id, with_features, env).await?;
        }
        RSpotifyCli::Search {
            title,
            artist,
            with_features,
        } => {
            cli::handler::handle_search_song(&title, &artist, with_features, env).await?;
        }
    }

    Ok(())
}
