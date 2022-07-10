use crate::v1::{Client, error, BASE_URL};

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct GetAccountResponse {
    /// The account returned in this response.
    pub data : Data,
}

#[derive(Deserialize, Debug)]
pub struct ListAccountsResponse {
    /// The list of accounts returned in this response.
    pub data : Vec<Data>,
    pub links : ResponseLinks,
}

#[derive(Deserialize, Debug)]
pub struct DataLinks {
    #[serde(rename = "self")]
    /// The canonical link to this resource within the API.
    pub this : Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct ResponseLinks {
    /// The link to the previous page in the results. If this value is `None` there is no previous page.
    pub prev : Option<String>,
    /// The link to the next page in the results. If this value is `None` there is no next page.
    pub next : Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Data {
    /// The type of this resource: `accounts`.
    pub r#type : String,
    /// The unique identifier for this account.
    pub id : String,
    pub attributes : Attributes,
    pub relationships : Relationships,
    pub links : Option<DataLinks>,
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
pub struct Attributes {
    #[serde(rename = "displayName")]
    /// The name associated with the account in the Up application.
    pub display_name : String,
    #[serde(rename = "accountType")]
    /// The bank account type of this account. Possible values: SAVER, TRANSACTIONAL
    pub account_type : String,
    #[serde(rename = "ownershipType")]
    /// The ownership structure for this account. Possible values: INDIVIDUAL, JOINT
    pub ownership_type : String,
    /// The available balance of the account, taking into account any amounts that are currently on hold.
    pub balance : Balance,
    #[serde(rename = "createdAt")]
    /// The date-time at which this account was first opened.
    pub created_at : String,
}

#[derive(Deserialize, Debug)]
pub struct Balance {
    #[serde(rename = "currencyCode")]
    /// The ISO 4217 currency code.
    pub currency_code : String,
    /// The amount of money, formatted as a string in the relevant currency. For example, for an Australian dollar value of $10.56, this field will be `"10.56"`. The currency symbol is not included in the string
    pub value : String,
    #[serde(rename = "valueInBaseUnits")]
    /// The amount of money in the smallest denomination for the currency, as a 64-bit integer. For example, for an Australian dollar value of $10.56, this field will be `1056`.
    pub value_in_base_units : i64,
}

#[derive(Default)]
pub struct ListAccountsOptions {
    /// The number of records to return in each page. 
    page_size : Option<u8>,
    /// The type of account for which to return records. This can be used to filter Savers from spending accounts.
    filter_account_type : Option<String>,
    /// The account ownership structure for which to return records. This can be used to filter 2Up accounts from Up accounts.
    filter_owership_type : Option<String>,
}

impl ListAccountsOptions {
    /// Sets the page size.
    pub fn page_size(&mut self, value : u8) {
        self.page_size = Some(value);
    }

    /// Sets the account type filter value.
    pub fn filter_account_type(&mut self, value : String) {
        self.filter_account_type = Some(value);
    }

    /// Sets the ownership type filter value.
    pub fn filter_owership_type(&mut self, value : String) {
        self.filter_owership_type = Some(value);
    }

    fn add_params(&self, url : &mut reqwest::Url) {
        if let Some(value) = &self.page_size {
            url.set_query(Some(&format!("page[size]={}", value)));
        }

        if let Some(value) = &self.filter_account_type {
            url.set_query(Some(&format!("filter[accountType]={}", value)));
        }

        if let Some(value) = &self.filter_owership_type {
            url.set_query(Some(&format!("filter[ownershipType]={}", value)));
        }
    }
}

impl Client {
    /// Retrieve a paginated list of all accounts for the currently authenticated user. The returned list is paginated and can be scrolled by following the `prev` and `next` links where present. 
    pub async fn list_accounts(&self, options : &ListAccountsOptions) -> Result<ListAccountsResponse, error::Error> {
        let mut url = reqwest::Url::parse(&format!("{}/accounts", BASE_URL)).map_err(error::Error::UrlParse)?;
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
                let account_response : ListAccountsResponse = serde_json::from_str(&body).map_err(error::Error::Json)?;

                Ok(account_response)
            },
            _ => {
                let body = res.text().await.map_err(error::Error::BodyRead)?;
                let error : error::ErrorResponse = serde_json::from_str(&body).map_err(error::Error::Json)?;

                Err(error::Error::Api(error))
            }
        }
    }

    /// Retrieve a specific account by providing its unique identifier.
    pub async fn get_account(&self, id : &str) -> Result<GetAccountResponse, error::Error> {
        // This assertion is because without an ID the request is thought to be a request for
        // many accounts, and therefore the error messages are very unclear.
        if id.is_empty() {
            panic!("The provided account ID must not be empty.");
        }

        let url = reqwest::Url::parse(&format!("{}/accounts/{}", BASE_URL, id)).map_err(error::Error::UrlParse)?;

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
                let account_response : GetAccountResponse = serde_json::from_str(&body).map_err(error::Error::Json)?;

                Ok(account_response)
            },
            _ => {
                let body = res.text().await.map_err(error::Error::BodyRead)?;
                let error : error::ErrorResponse = serde_json::from_str(&body).map_err(error::Error::Json)?;

                Err(error::Error::Api(error))
            }
        }
    }
}
