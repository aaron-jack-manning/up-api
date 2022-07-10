use crate::v1::{Client, error, BASE_URL};

use serde::{Deserialize, Serialize};

// ----------------- Response Objects -----------------

#[derive(Deserialize, Debug)]
pub struct ListCategoriesResponse {
    /// The list of categories returned in this response.
    pub data : Vec<CategoryResource>,
}

#[derive(Deserialize, Debug)]
pub struct GetCategoryResponse {
    /// The category returned in this response.
    pub data : CategoryResource,

}

#[derive(Deserialize, Debug)]
pub struct CategoryResource {
    /// The type of this resource: categories
    pub r#type : String,
    /// The unique identifier for this category. This is a human-readable but URL-safe value.
    pub id : String,
    pub attributes : Attributes,
    pub relationships : Relationships,
    pub links : Option<CategoryResourceLinks>,
}

#[derive(Deserialize, Debug)]
pub struct Attributes {
    /// The name of this category as seen in the Up application.
    pub name : String,
}

#[derive(Deserialize, Debug)]
pub struct Relationships {
    pub parent : Parent,
    pub children : Children,
}

#[derive(Deserialize, Debug)]
pub struct Parent {
    pub data : Option<ParentData>,
    pub links : Option<ParentLinks>,
}

#[derive(Deserialize, Debug)]
pub struct ParentData {
    /// The type of this resource: `categories`
    pub r#type : String,
    /// The unique identifier of the resource within its type.
    pub id : String,
}

#[derive(Deserialize, Debug)]
pub struct ParentLinks {
    /// The link to retrieve the related resource(s) in this relationship.
    pub related : String,
}

#[derive(Deserialize, Debug)]
pub struct Children {
    pub data : Vec<ChildrenData>,
    pub links : Option<ChildrenLinks>,
}

#[derive(Deserialize, Debug)]
pub struct ChildrenData {
    /// The type of this resource: `categories`
    pub r#type : String,
    /// The unique identifier of the resource within its type.
    pub id : String,
}

#[derive(Deserialize, Debug)]
pub struct ChildrenLinks {
    /// The link to retrieve the related resource(s) in this relationship.
    pub related : String,
}

#[derive(Deserialize, Debug)]
pub struct CategoryResourceLinks {
    /// The canonical link to this resource within the API.
    #[serde(rename = "self")]
    pub this : String,
}

// ----------------- Input Objects -----------------

#[derive(Default)]
pub struct ListCategoriesOptions {
    /// The unique identifier of a parent category for which to return only its children.
    filter_parent : Option<String>,
}

impl ListCategoriesOptions {
    /// Sets the parent filter value.
    pub fn filter_parent(&mut self, value : String) {
        self.filter_parent = Some(value);
    }

    fn add_params(&self, url : &mut reqwest::Url) {
        let mut query = String::new();

        if let Some(value) = &self.filter_parent {
            if !query.is_empty() {
                query.push('&');
            }
            query.push_str(&format!("filter[parent]={}", value));
        }

        if !query.is_empty() {
            url.set_query(Some(&query));
        }
    }
}

// ----------------- Input Objects -----------------

#[derive(Serialize)]
struct CategoriseTransactionRequest {
    /// The category to set on the transaction. Set this entire key to `null` de-categorize a transaction.
    data : Option<CategoryInputResourceIdentifier>,
}

#[derive(Serialize)]
struct CategoryInputResourceIdentifier {
    /// The type of this resource: `categories`
    r#type : String,
    /// The unique identifier of the category, as returned by the `list_categories` method.
    id : String,
}

impl Client {
    /// Retrieve a list of all categories and their ancestry. The returned list is not paginated.
    pub async fn list_categories(&self, options : &ListCategoriesOptions) -> Result<ListCategoriesResponse, error::Error> {
        let mut url = reqwest::Url::parse(&format!("{}/categories", BASE_URL)).map_err(error::Error::UrlParse)?;
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
                let category_response : ListCategoriesResponse = serde_json::from_str(&body).map_err(error::Error::Json)?;

                Ok(category_response)
            },
            _ => {
                let body = res.text().await.map_err(error::Error::BodyRead)?;
                let error : error::ErrorResponse = serde_json::from_str(&body).map_err(error::Error::Json)?;

                Err(error::Error::Api(error))
            }
        }
    }

    /// Retrieve a specific category by providing its unique identifier.
    pub async fn get_category(&self, id : &str) -> Result<GetCategoryResponse, error::Error> {
        // This assertion is because without an ID the request is thought to be a request for
        // many accounts, and therefore the error messages are very unclear.
        if id.is_empty() {
            panic!("The provided account ID must not be empty.");
        }

        let url = reqwest::Url::parse(&format!("{}/categories/{}", BASE_URL, id)).map_err(error::Error::UrlParse)?;

        let res = reqwest::Client::new()
            .get(url)
            .header("Authorization", self.auth_header())
            .send()
            .await
            .map_err(error::Error::Request)?;

        match res.status() {
            reqwest::StatusCode::OK => {
                let body = res.text().await.map_err(error::Error::BodyRead)?;
                let category_response : GetCategoryResponse = serde_json::from_str(&body).map_err(error::Error::Json)?;

                Ok(category_response )
            },
            _ => {
                let body = res.text().await.map_err(error::Error::BodyRead)?;
                let error : error::ErrorResponse = serde_json::from_str(&body).map_err(error::Error::Json)?;

                Err(error::Error::Api(error))
            }
        }
    }

    /// Updates the category associated with a transaction. Only transactions for which `is_categorizable` is set to true support this operation. The `id` is taken from the list exposed on `list_categories` and cannot be one of the top-level (parent) categories. To de-categorize a transaction, set the entire `data` key to `null`. The associated category, along with its request URL is also exposed via the category relationship on the transaction resource returned from `get_transaction`.
    pub async fn categorise_transaction(&self, transaction_id : &str, category : Option<&str>) -> Result<(), error::Error> {
        let url = reqwest::Url::parse(&format!("{}/transactions/{}/relationships/category", BASE_URL, transaction_id)).map_err(error::Error::UrlParse)?;

        let category = category.map(|id| {
            CategoryInputResourceIdentifier {
                r#type : String::from("categories"),
                id : String::from(id),
            }
        });

        let body = CategoriseTransactionRequest  { data : category };
        let body = serde_json::to_string(&body).map_err(error::Error::Serialize)?;

        println!("{}", body);

        let res = reqwest::Client::new()
            .patch(url)
            .header("Authorization", self.auth_header())
            .header("Content-Type", "application/json")
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
