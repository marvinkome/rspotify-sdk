use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ClientAuthorizeResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u32,
}
