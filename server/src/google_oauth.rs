use std::error::Error;

use reqwest::Client;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct OauthResponse {
    pub access_token: String,
    pub id_token: String,
}

pub async fn get_access_token(auth_code: &str) -> Result<OauthResponse, Box<dyn Error>> {
    let http_client = Client::new();
    let root_url = "https://oauth2.googleapis.com/token";
    let redirect_url = "http://localhost:6969/auth/google".to_string();
    let client_id =
        "899470897400-4qubqe3mancekjuj7a5hfq2acn0ecbmf.apps.googleusercontent.com".to_string();
    let client_secret = "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".to_string();

    let params = [
        ("grant_type", "authorization_code"),
        ("redirect_uri", &redirect_url),
        ("client_id", &client_id),
        ("client_secret", &client_secret),
        ("code", auth_code),
    ];

    let resp = http_client.post(root_url).form(&params).send().await?;

    if resp.status().is_success() {
        let oauth_response = resp.json::<OauthResponse>().await?;
        return Ok(oauth_response);
    } else {

        println!("access token res: {:?}", resp);
        return Err(From::from("could not retrieve access token: {}"));
    }
}

pub async fn get_google_user(){
    
}
