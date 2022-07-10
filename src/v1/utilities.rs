use crate::v1::{Client, error, BASE_URL};

use serde::Deserialize;

// ----------------- Request Objects -----------------

#[derive(Deserialize, Debug)]
pub struct PingResponse {
    pub meta : Meta,
}

#[derive(Deserialize, Debug)]
pub struct Meta {
    /// The unique identifier of the authenticated customer.
    pub id : String,
    #[serde(rename = "statusEmoji")]
    /// A cute emoji that represents the response status.
    pub status_emoji : String,
}

impl Client {
    /// Make a basic ping request to the API. This is useful to verify that authentication is functioning correctly.
    pub async fn ping(&self) -> Result<PingResponse, error::Error> {
        let url = reqwest::Url::parse(&format!("{}/util/ping", BASE_URL)).map_err(error::Error::UrlParse)?;

        let res = reqwest::Client::new()
            .get(url)
            .header("Authorization", self.auth_header())
            .send()
            .await
            .map_err(error::Error::Request)?;

        match res.status() {
            reqwest::StatusCode::OK => {
                let body = res.text().await.map_err(error::Error::BodyRead)?;
                println!("{}", body);
                let ping_response : PingResponse = serde_json::from_str(&body).map_err(error::Error::Json)?;

                Ok(ping_response)
            },
            _ => {
                let body = res.text().await.map_err(error::Error::BodyRead)?;
                let error : error::ErrorResponse = serde_json::from_str(&body).map_err(error::Error::Json)?;

                Err(error::Error::Api(error))
            }
        }
    }
}
