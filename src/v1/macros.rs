macro_rules! implement_pagination_v1 {
    ($t:ty) => {
        impl $t {
            async fn follow_link(client : &Client, url : &str) -> Result<Self, error::Error> {
                let res = reqwest::Client::new()
                    .get(url)
                    .header("Authorization", client.auth_header())
                    .send()
                    .await
                    .map_err(error::Error::Request)?;

                match res.status() {
                    reqwest::StatusCode::OK => {
                        let body = res.text().await.map_err(error::Error::BodyRead)?;
                        let response : Self = serde_json::from_str(&body).map_err(error::Error::Json)?;

                        Ok(response)
                    },
                    _ => {
                        let body = res.text().await.map_err(error::Error::BodyRead)?;
                        let error : error::ErrorResponse = serde_json::from_str(&body).map_err(error::Error::Json)?;

                        Err(error::Error::Api(error))
                    }
                }
            }

            /// Follows the link to the next page, returns None of the next page does not exist.
            pub async fn next(&self, client : &Client) -> Option<Result<Self, error::Error>> {
                match
                    self
                    .links
                    .next
                    .as_ref()
                    .map(|url| Self::follow_link(client, &url)) {
                        
                    Some(data) => Some(data.await),
                    None => None,
                }
            }

            /// Follows the link to the previous page, returns None of the previous page does not exist.
            pub async fn prev(&self, client : &Client) -> Option<Result<Self, error::Error>> {
                match
                    self
                    .links
                    .prev
                    .as_ref()
                    .map(|url| Self::follow_link(client, &url)) {
                        
                    Some(data) => Some(data.await),
                    None => None,
                }
            }
        }
    }
}


