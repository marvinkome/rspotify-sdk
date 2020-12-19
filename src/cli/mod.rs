pub mod handler;
pub mod user;

use structopt::StructOpt;

/// CLI application for getting data from spotify api
#[derive(StructOpt, Debug)]
#[structopt(name = "rspotify", about = "CLI for getting data from spotify api")]
pub enum RSpotifyCli {
    /// Get all tracks from a playlist
    GetPlaylistTracks {
        /// playlist id
        id: String,

        /// adds the tracks features to the response
        #[structopt(long = "with-features")]
        with_features: bool,
    },

    /// Get all tracks from a album
    GetAlbumTracks {
        /// album id
        id: String,

        /// adds the tracks features to the response
        #[structopt(long = "with-features")]
        with_features: bool,
    },

    /// Search for a track
    Search {
        /// track
        #[structopt(long = "title")]
        title: String,

        /// artist
        #[structopt(long = "artist")]
        artist: String,

        /// adds the tracks features to the response
        #[structopt(long = "with-features")]
        with_features: bool,
    },

    /// Get all user playlists
    GetMyPlaylists {
        /// adds the tracks features to the response
        #[structopt(long = "with-features")]
        with_features: bool,
    },

    /// Get all user albums
    GetMyAlbums {
        /// adds the tracks features to the response
        #[structopt(long = "with-features")]
        with_features: bool,
    },

    /// Get all user liked songs
    GetMyLikedSongs {
        /// adds the tracks features to the response
        #[structopt(long = "with-features")]
        with_features: bool,
    },

    /// Get all user data (playlists, liked songs, followed albums)
    GetMyData {
        /// adds the tracks features to the response
        #[structopt(long = "with-features")]
        with_features: bool,
    },
}
