use crate::v1::{Client, error, BASE_URL, standard};

use serde::Deserialize;

// ----------------- Response Objects -----------------

#[derive(Deserialize, Debug)]
pub struct ListTransactionsResponse {
    /// The list of transactions returned in this response.
    pub data : Vec<TransactionResource>,
    pub links : ResponseLinks,
}

#[derive(Deserialize, Debug)]
pub struct GetTransactionResponse {
    /// The transaction returned in this response.
    pub data : TransactionResource,
}

#[derive(Deserialize, Debug)]
pub struct TransactionResource {
    /// The type of this resource: `transactions`
    pub r#type : String,
    /// The unique identifier for this transaction.
    pub id : String,
    pub attributes : Attributes,
    pub relationships : Relationships,
    pub links : Option<TransactionResourceLinks>,
}

#[derive(Deserialize, Debug)]
pub struct TransactionResourceLinks {
    /// The canonical link to this resource within the API.
    #[serde(rename = "self")]
    pub this : String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Relationships {
    pub account : Account,
    /// If this transaction is a transfer between accounts, this relationship will contain the account the transaction went to/came from. The `amount` field can be used to determine the direction of the transfer.
    pub transfer_account : TransferAccount,
    pub category : Category,
    pub parent_category : ParentCategory,
    pub tags : Tags,
}

#[derive(Deserialize, Debug)]
pub struct Account {
    pub data : AccountData,
    pub links : Option<AccountLinks>,
}

#[derive(Deserialize, Debug)]
pub struct AccountData {
    /// The type of this resource: `accounts`
    pub r#type : String,
    /// The unique identifier of the resource within its type.
    pub id : String,
}

#[derive(Deserialize, Debug)]
pub struct AccountLinks {
    /// The link to retrieve the related resource(s) in this relationship.
    pub related : String,
}

#[derive(Deserialize, Debug)]
pub struct TransferAccount {
    pub data : Option<AccountData>,
    pub links : Option<AccountLinks>,
}

#[derive(Deserialize, Debug)]
pub struct TransferAccountData {
    /// The type of this resource: `accounts`
    pub r#type : String,
    /// The unique identifier of the resource within its type.
    pub id : String,
}

#[derive(Deserialize, Debug)]
pub struct TransferAccountLinks {
    /// The link to retrieve the related resource(s) in this relationship.
    pub related : String,
}

#[derive(Deserialize, Debug)]
pub struct Category {
    pub data : Option<CategoryData>,
    pub links : Option<CategoryLinks>,
}

#[derive(Deserialize, Debug)]
pub struct CategoryData {
    /// The type of this resource: `categories`
    pub r#type : String,
    /// The unique identifier of the resource within its type.
    pub id : String,
}

#[derive(Deserialize, Debug)]
pub struct CategoryLinks {
    /// The link to retrieve or modify linkage between this resources and the related resource(s) in this relationship.
    #[serde(rename = "self")]
    pub this : String,
    pub related : Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct ParentCategory {
    pub data : Option<ParentCategoryData>,
    pub links : Option<ParentCategoryLinks>,
}

#[derive(Deserialize, Debug)]
pub struct ParentCategoryData {
    /// The type of this resource: `categories`
    pub r#type : String,
    /// The unique identifier of the resource within its type.
    pub id : String,
}

#[derive(Deserialize, Debug)]
pub struct ParentCategoryLinks {
    /// The link to retrieve the related resource(s) in this relationship.
    pub related : String,
}

#[derive(Deserialize, Debug)]
pub struct Tags {
    pub data : Vec<TagsData>,
    pub links : Option<TagsLinks>,
}

#[derive(Deserialize, Debug)]
pub struct TagsData {
    /// The type of this resource: `tags`
    pub r#type : String,
    /// The label of the tag, which also acts as the tag’s unique identifier.
    pub id : String,
}

#[derive(Deserialize, Debug)]
pub struct TagsLinks {
    /// The link to retrieve or modify linkage between this resources and the related resource(s) in this relationship.
    #[serde(rename = "self")]
    pub this : String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    /// The current processing status of this transaction, according to whether or not this transaction has settled or is still held. Possible values: `HELD`, `SETTLED`
    pub status : standard::TransactionStatusEnum,
    /// The original, unprocessed text of the transaction. This is often not a perfect indicator of the actual merchant, but it is useful for reconciliation purposes in some cases.
    pub raw_text : Option<String>,
    /// A short description for this transaction. Usually the merchant name for purchases.
    pub description : String,
    /// Attached message for this transaction, such as a payment message, or a transfer note.
    pub message : Option<String>,
    /// Boolean flag set to true on transactions that support the use of categories.
    pub is_categorizable : bool,
    /// If this transaction is currently in the `HELD` status, or was ever in the `HELD` status, the `amount` and `foreignAmount` of the transaction while `HELD`.
    pub hold_info : Option<standard::HoldInfoObject>,
    /// Details of how this transaction was rounded-up. If no Round Up was applied this field will be `null`.
    pub round_up : Option<standard::RoundUpObject>,
    /// If all or part of this transaction was instantly reimbursed in the form of cashback, details of the reimbursement.
    pub cashback : Option<standard::CashBackObject>,
    /// The amount of this transaction in Australian dollars. For transactions that were once `HELD` but are now `SETTLED`, refer to the `holdInfo` field for the original `amount` the transaction was `HELD` at.
    pub amount : standard::MoneyObject,
    /// The foreign currency amount of this transaction. This field will be `null` for domestic transactions. The amount was converted to the AUD amount reflected in the `amount` of this transaction. Refer to the `holdInfo` field for the original foreignAmount the transaction was `HELD` at.
    pub foreign_amount : Option<standard::MoneyObject>,
    /// Information about the card used for this transaction, if applicable.
    pub card_purchase_method : Option<standard::CardPurchaseMethodObject>,
    /// The date-time at which this transaction settled. This field will be `null` for transactions that are currently in the `HELD` status.
    pub settled_at : Option<String>,
    /// The date-time at which this transaction was first encountered.
    pub created_at : String,
}

#[derive(Deserialize, Debug)]
pub struct ResponseLinks {
    /// The link to the previous page in the results. If this value is null there is no previous page.
    pub prev : Option<String>,
    /// The link to the next page in the results. If this value is null there is no next page.
    pub next : Option<String>,
}

// ----------------- Input Objects -----------------

#[derive(Default)]
pub struct ListTransactionsOptions {
    /// The number of records to return in each page. 
    page_size : Option<u8>,
    /// The transaction status for which to return records. This can be used to filter `HELD` transactions from those that are `SETTLED`.
    filter_status : Option<String>,
    /// The start date-time from which to return records, formatted according to rfc-3339. Not to be used for pagination purposes.
    filter_since : Option<String>,
    /// The end date-time up to which to return records, formatted according to rfc-3339. Not to be used for pagination purposes.
    filter_until : Option<String>,
    /// The category identifier for which to filter transactions. Both parent and child categories can be filtered through this parameter.
    filter_category : Option<String>,
    /// A transaction tag to filter for which to return records. If the tag does not exist, zero records are returned and a success response is given.
    filter_tag : Option<String>,
}

impl ListTransactionsOptions {
    /// Sets the page size.
    pub fn page_size(&mut self, value : u8) {
        self.page_size = Some(value);
    }

    /// Sets the status filter value.
    pub fn filter_status(&mut self, value : String) {
        self.filter_status = Some(value);
    }

    /// Sets the since filter value.
    pub fn filter_since(&mut self, value : String) {
        self.filter_since = Some(value);
    }

    /// Sets the until filter value.
    pub fn filter_until (&mut self, value : String) {
        self.filter_until = Some(value);
    }

    /// Sets the category filter value.
    pub fn filter_category(&mut self, value : String) {
        self.filter_category = Some(value);
    }

    /// Sets the tag filter value.
    pub fn filter_tag (&mut self, value : String) {
        self.filter_tag = Some(value);
    }

    fn add_params(&self, url : &mut reqwest::Url) {
        let mut query = String::new();

        if let Some(value) = &self.page_size {
            if !query.is_empty() {
                query.push('&');
            }
            query.push_str(&format!("page[size]={}", value));
        }

        if let Some(value) = &self.filter_status {
            if !query.is_empty() {
                query.push('&');
            }
            query.push_str(&format!("filter[status]={}", value));
        }

        if let Some(value) = &self.filter_since {
            if !query.is_empty() {
                query.push('&');
            }
            query.push_str(&format!("filter[since]={}", value));
        }

        if let Some(value) = &self.filter_until {
            if !query.is_empty() {
                query.push('&');
            }
            query.push_str(&format!("filter[until]={}", value));
        }

        if let Some(value) = &self.filter_category {
            if !query.is_empty() {
                query.push('&');
            }
            query.push_str(&format!("filter[category]={}", value));
        }

        if let Some(value) = &self.filter_tag {
            if !query.is_empty() {
                query.push('&');
            }
            query.push_str(&format!("filter[tag]={}", value));
        }

        if !query.is_empty() {
            url.set_query(Some(&query));
        }
    }
}

impl Client {
    /// Retrieve a list of all transactions across all accounts for the currently authenticated user. The returned list is paginated and can be scrolled by following the `next` and `prev` links where present. To narrow the results to a specific date range pass one or both of `filter[since]` and `filter[until]` in the query string. These filter parameters should not be used for pagination. Results are ordered newest first to oldest last.
    pub async fn list_transactions(&self, options : &ListTransactionsOptions) -> Result<ListTransactionsResponse, error::Error> {
        let mut url = reqwest::Url::parse(&format!("{}/transactions", BASE_URL)).map_err(error::Error::UrlParse)?;
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
                let transaction_response : ListTransactionsResponse = serde_json::from_str(&body).map_err(error::Error::Json)?;

                Ok(transaction_response)
            },
            _ => {
                let body = res.text().await.map_err(error::Error::BodyRead)?;
                let error : error::ErrorResponse = serde_json::from_str(&body).map_err(error::Error::Json)?;

                Err(error::Error::Api(error))
            }
        }
    }

    /// Retrieve a specific transaction by providing its unique identifier.
    pub async fn get_transaction(&self, id : &String) -> Result<GetTransactionResponse, error::Error> {
        // This assertion is because without an ID the request is thought to be a request for
        // many transactions, and therefore the error messages are very unclear.
        if id.is_empty() {
            panic!("The provided transaction ID must not be empty.");
        }

        let url = reqwest::Url::parse(&format!("{}/transactions/{}", BASE_URL, id)).map_err(error::Error::UrlParse)?;

        let res = reqwest::Client::new()
            .get(url)
            .header("Authorization", self.auth_header())
            .send()
            .await
            .map_err(error::Error::Request)?;

        match res.status() {
            reqwest::StatusCode::OK => {
                let body = res.text().await.map_err(error::Error::BodyRead)?;
                let transaction_response : GetTransactionResponse = serde_json::from_str(&body).map_err(error::Error::Json)?;

                Ok(transaction_response)
            },
            _ => {
                let body = res.text().await.map_err(error::Error::BodyRead)?;
                let error : error::ErrorResponse = serde_json::from_str(&body).map_err(error::Error::Json)?;

                Err(error::Error::Api(error))
            }
        }
    }

    /// Retrieve a list of all transactions for a specific account. The returned list is paginated and can be scrolled by following the `next` and `prev` links where present. To narrow the results to a specific date range pass one or both of `filter[since]` and `filter[until]` in the query string. These filter parameters should not be used for pagination. Results are ordered newest first to oldest last.
    pub async fn list_transactions_by_account(&self, account_id : &String, options : &ListTransactionsOptions) -> Result<ListTransactionsResponse, error::Error> {
        let mut url = reqwest::Url::parse(&format!("{}/accounts/{}/transactions", BASE_URL, account_id)).map_err(error::Error::UrlParse)?;
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
                let transaction_response : ListTransactionsResponse = serde_json::from_str(&body).map_err(error::Error::Json)?;

                Ok(transaction_response)
            },
            _ => {
                let body = res.text().await.map_err(error::Error::BodyRead)?;
                let error : error::ErrorResponse = serde_json::from_str(&body).map_err(error::Error::Json)?;

                Err(error::Error::Api(error))
            }
        }
    }
}


// ----------------- Page Navigation -----------------

implement_pagination_v1!(ListTransactionsResponse);
