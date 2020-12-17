pub mod helpers;
pub mod requests;

use log::info;
use std::collections::HashMap;
use std::io::prelude::*;
use std::io::{Error, ErrorKind, BufWriter};
use std::fs::File;
use std::path::Path;
use std::net::{TcpListener, TcpStream};
use url::Url;


const SPOTIFY_AUTHORIZATION_URL: &'static str = "https://accounts.spotify.com/authorize";

fn handle_request(mut stream: TcpStream, spotify_url: &str) -> Result<String, ()> {
    let this_server = Url::parse("http://127.0.0.1:8008").unwrap();
    // read request
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    // get url from request
    let mut headers = [httparse::EMPTY_HEADER; 16];
    let mut req = httparse::Request::new(&mut headers);
    req.parse(&buffer[..]).unwrap();
    let path = req.path.unwrap();
    let url = this_server.join(&path).unwrap();

    // match paths
    match url.path() {
        "/" => {
            let response = format!("HTTP/1.1 302 OK\r\nLocation: {}\r\n", spotify_url);

            stream.write(response.as_bytes()).unwrap();
            stream.flush().unwrap();
            Err(())
        }
        "/callback" => {
            // parse query
            let query: HashMap<_, _> = url.query_pairs().into_owned().collect();
            let code = match query.get("code") {
                Some(code) => code,
                None => {
                    let contents = "Authorization unsuccessful code not found";

                    let response = format!(
                        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
                        contents.len(),
                        contents
                    );

                    stream.write(response.as_bytes()).unwrap();
                    stream.flush().unwrap();
                    return Err(());
                }
            };
            let contents = "Authorization successful";
            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
                contents.len(),
                contents
            );

            stream.write(response.as_bytes()).unwrap();
            stream.flush().unwrap();

            return Ok(code.to_owned());
        }
        _ => {
            let contents = "404 page";

            let response = format!(
                "HTTP/1.1 404 NOT-FOUND\r\nContent-Length: {}\r\n\r\n{}",
                contents.len(),
                contents
            );

            stream.write(response.as_bytes()).unwrap();
            stream.flush().unwrap();
            Err(())
        }
    }
}

pub fn open_browser_for_auth(
    client_id: &str,
    scope: &str,
    show_dialog: bool,
) -> Result<String, ()> {
    let spotify_url = format!(
        "{}?client_id={}&response_type=code&redirect_uri=http://loca\
    lhost:8008/callback&scope={}&show_dialog={}",
        SPOTIFY_AUTHORIZATION_URL, client_id, scope, show_dialog
    );

    let listener = TcpListener::bind("127.0.0.1:8008").unwrap();
    info!("TCP listener listening on port 8008");

    match std::process::Command::new("open")
        .arg("http://127.0.0.1:8008")
        .spawn()
    {
        Ok(_s) => {}
        Err(_e) => panic!("Failed to open browser"),
    };

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        match handle_request(stream, &spotify_url) {
            Ok(code) => {
                info!("Closing TCP listener on port 8008");
                return Ok(code);
            }
            Err(_e) => {
                continue;
            }
        };
    }

    Err(())
}

pub fn read_from_auth_cache(key: &str) -> std::io::Result<String> {
    let file = std::fs::read_to_string("auth.json")?;
    let json = serde_json::from_str::<HashMap<String, String>>(&file)?;

    let data = match json.get(key) {
        Some(value) => value,
        None => {
            let err = Error::new(ErrorKind::Other, "key not found");
            return Err(err);
        }
    };

    Ok(data.to_owned())
}

pub fn write_to_auth_cache(key: &str, value: &str) -> std::io::Result<()> {
    // read current cache items to json
    let path = Path::new("auth.json");
    let file = match path.exists() {
        true => {
            std::fs::read_to_string("auth.json").unwrap()
        }
        false => {
            let mut buffer = String::new();
            File::create("auth.json").unwrap().read_to_string(&mut buffer)?;
            buffer
        }
    };


    let mut cache = match file.len() {
        0 => HashMap::new(),
        _ => serde_json::from_str::<HashMap<&str, &str>>(&file)?,
    };

    cache.insert(key, value);

    let json_resp = serde_json::to_string(&cache)?;
    let write_file = File::create("auth.json")?;
    let mut write_file = BufWriter::new(write_file);
    write_file.write_all(json_resp.as_bytes())?;
    // std::fs::write(path, json_resp)?;

    Ok(())
}
