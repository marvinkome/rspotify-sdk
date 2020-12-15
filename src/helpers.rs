use reqwest::{header, Client};

pub fn generate_request(token: &str) -> (Client, header::HeaderMap) {
    let client = Client::new();

    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::AUTHORIZATION,
        header::HeaderValue::from_str(&format!("Bearer {}", token)).unwrap(),
    );

    return (client, headers);
}
