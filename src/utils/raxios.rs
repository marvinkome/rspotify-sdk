use reqwest::{header, Client, Error};
use serde::Deserialize;
use std::collections::HashMap;

pub struct RaxiosClient {
    token: Option<String>,
}

impl RaxiosClient {
    pub fn new() -> Self {
        RaxiosClient { token: None }
    }

    pub fn set_token(&mut self, token: &str) {
        self.token = Some(token.to_owned());
    }

    pub async fn get<T: for<'de> Deserialize<'de>>(
        &self,
        url: &str,
        headers: Option<header::HeaderMap>,
    ) -> Result<T, Error> {
        let client = Client::new();
        let request_builder = client.get(url);

        let request_builder = match &self.token {
            None => request_builder,
            Some(token) => request_builder.bearer_auth(token),
        };

        let request_builder = match headers {
            None => request_builder,
            Some(headers) => request_builder.headers(headers),
        };

        let response = request_builder.send().await?;
        response.json::<T>().await
    }

    pub async fn _post<T: for<'de> Deserialize<'de>>(
        &self,
        url: &str,
        body: Option<HashMap<String, String>>,
        headers: Option<header::HeaderMap>,
    ) -> Result<T, Error> {
        let client = Client::new();
        let request_builder = client.get(url);

        let request_builder = match body {
            None => request_builder,
            Some(body) => request_builder.json(&body),
        };

        let request_builder = match &self.token {
            None => request_builder,
            Some(token) => request_builder.bearer_auth(token),
        };

        let request_builder = match headers {
            None => request_builder,
            Some(headers) => request_builder.headers(headers),
        };

        let response = request_builder.send().await?;
        response.json::<T>().await
    }
}
