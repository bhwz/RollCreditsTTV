use serde::{Deserialize, Serialize};

pub async fn request_s2s_token() -> Result<AccessToken, reqwest::Error> {
    // Temp for testing
    // let TTV_ENDPOINT = "https://id.twitch.tv/oauth2/token";
    const TTV_ENDPOINT: &str = "https://httpbin.org/status/200";

    // TODO: Pull from env
    let client_credentials = ClientCredentials {
        client_id: "dummy".to_string(),
        client_secret: "dummy".to_string(),
        grant_type: "client_credentials".to_string(),
    };

    let req_client = reqwest::Client::new();

    req_client.post(TTV_ENDPOINT).form(&client_credentials).send().await?.json::<AccessToken>().await
}

#[derive(Serialize)]
pub struct ClientCredentials {
    client_id: String,
    client_secret: String,
    grant_type: String,
}

#[derive(Deserialize, Debug)]
pub struct AccessToken {
    access_token: String,
    expires_in: u64,
    token_type: String,
}
