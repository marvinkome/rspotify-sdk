use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ClientAuthorizeResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u32,
}

#[derive(Deserialize, Debug)]
pub struct UserAuthorizeResponse {
    pub access_token: String,
    pub token_type: String,
    pub scope: String,
    pub expires_in: u32,
    pub refresh_token: Option<String>,
}
