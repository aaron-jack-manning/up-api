use crate::v1::{Client, error, BASE_URL};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct ListTagsResponse {
    /// The list of tags returned in this response.
    pub data : Vec<Data>,
    pub links : ResponseLinks,
}

#[derive(Deserialize, Debug)]
pub struct Data {
    /// The type of this resource: `tags`
    pub r#type : String,
    /// The label of the tag, which also acts as the tag’s unique identifier.
    pub id : String,
    pub relationships : Relationships,
}

#[derive(Deserialize, Debug)]
pub struct Relationships {
    pub transactions : Transactions,
}

#[derive(Deserialize, Debug)]
pub struct Transactions {
    pub links : Option<TransactionLinks>,
}

#[derive(Deserialize, Debug)]
pub struct TransactionLinks {
    /// The link to retrieve the related resource(s) in this relationship.
    pub related : String,
}

#[derive(Deserialize, Debug)]
pub struct ResponseLinks {
    pub prev : Option<String>,
    pub next : Option<String>,
}

#[derive(Default)]
pub struct ListTagsOptions {
    /// The number of records to return in each page. 
    page_size : Option<u8>,
}

impl ListTagsOptions {
    /// Sets the page size.
    pub fn page_size(&mut self, value : u8) {
        self.page_size = Some(value);
    }

    fn add_params(&self, url : &mut reqwest::Url) {
        if let Some(value) = &self.page_size {
            url.set_query(Some(&format!("page[size]={}", value)));
        }
    }
}

#[derive(Serialize)]
struct TagInputResourceIdentifier {
    /// The type of this resource: `tags`
    r#type : String,
    /// The label of the tag, which also acts as the tag’s unique identifier.
    id : String,
}

#[derive(Serialize)]
struct TagRequest {
    /// The tags to add to or remove from the transaction.
    data : Vec<TagInputResourceIdentifier>
}


impl Client {
    /// Retrieve a list of all tags currently in use. The returned list is paginated and can be scrolled by following the `next` and `prev`  links where present. Results are ordered lexicographically. The transactions relationship for each tag exposes a link to get the transactions with the given tag.
    pub async fn list_tags(&self, options : &ListTagsOptions) -> Result<ListTagsResponse, error::Error> {
        let mut url = reqwest::Url::parse(&format!("{}/tags", BASE_URL)).map_err(error::Error::UrlParse)?;
        options.add_params(&mut url);

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
                let tags_response : ListTagsResponse = serde_json::from_str(&body).map_err(error::Error::Json)?;

                Ok(tags_response)
            },
            _ => {
                let body = res.text().await.map_err(error::Error::BodyRead)?;
                let error : error::ErrorResponse = serde_json::from_str(&body).map_err(error::Error::Json)?;

                Err(error::Error::Api(error))
            }
        }
    }

    /// Associates one or more tags with a specific transaction. No more than 6 tags may be present on any single transaction. Duplicate tags are silently ignored. The associated tags, along with this request URL, are also exposed via the tags relationship on the transaction resource returned from `get_transaction`.
    pub async fn add_tags(&self, transaction_id : &str, tags : Vec<String>) -> Result<(), error::Error> {
        let url = reqwest::Url::parse(&format!("{}/transactions/{}/relationships/tags", BASE_URL, transaction_id)).map_err(error::Error::UrlParse)?;

        let tags =
            tags
            .into_iter()
            .map(|t| TagInputResourceIdentifier {
                r#type : String::from("tags"),
                id : t
            })
            .collect();

        let body = TagRequest { data : tags };
        let body = serde_json::to_string(&body).map_err(error::Error::Serialize)?;

        let res = reqwest::Client::new()
            .post(url)
            .header("Authorization", self.auth_header())
            .body(body)
            .send()
            .await
            .map_err(error::Error::Request)?;

        match res.status() {
            reqwest::StatusCode::NO_CONTENT => {
                Ok(())
            },
            _ => {
                let body = res.text().await.map_err(error::Error::BodyRead)?;
                let error : error::ErrorResponse = serde_json::from_str(&body).map_err(error::Error::Json)?;

                Err(error::Error::Api(error))
            }
        }
    }

    /// Disassociates one or more tags from a specific transaction. Tags that are not associated are silently ignored. The associated tags, along with this request URL, are also exposed via the tags relationship on the transaction resource returned from `get_transaction`.
    pub async fn delete_tags(&self, transaction_id : &str, tags : Vec<String>) -> Result<(), error::Error> {
        let url = reqwest::Url::parse(&format!("{}/transactions/{}/relationships/tags", BASE_URL, transaction_id)).map_err(error::Error::UrlParse)?;

        let tags =
            tags
            .into_iter()
            .map(|t| TagInputResourceIdentifier {
                r#type : String::from("tags"),
                id : t
            })
            .collect();

        let body = TagRequest { data : tags };
        let body = serde_json::to_string(&body).map_err(error::Error::Serialize)?;

        let res = reqwest::Client::new()
            .delete(url)
            .header("Authorization", self.auth_header())
            .body(body)
            .send()
            .await
            .map_err(error::Error::Request)?;

        match res.status() {
            reqwest::StatusCode::NO_CONTENT => {
                Ok(())
            },
            _ => {
                let body = res.text().await.map_err(error::Error::BodyRead)?;
                let error : error::ErrorResponse = serde_json::from_str(&body).map_err(error::Error::Json)?;

                Err(error::Error::Api(error))
            }
        }
    }
}

