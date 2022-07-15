use crate::v1::{Client, error, BASE_URL, standard};

use serde::{Deserialize, Serialize};

// ----------------- Response Objects -----------------

#[derive(Deserialize, Debug)]
pub struct ListWebhooksResponse {
    /// The list of webhooks returned in this response.
    pub data : Vec<WebhookResource>,
    pub links : ResponseLinks,
}

#[derive(Deserialize, Debug)]
pub struct GetWebhookResponse {
    /// The webhook returned in the response.
    pub data : WebhookResource,
}

#[derive(Deserialize, Debug)]
pub struct CreateWebhookResponse {
    /// The webhook that was created.
    pub data : WebhookResource,
}

#[derive(Deserialize, Debug)]
pub struct WebhookResource {
    /// The type of this resource: `webhooks`
    pub r#type : String,
    /// The unique identifier for this webhook.
    pub id : String,
    pub attributes : Attributes,
    pub relationships : Relationships,
    pub links : WebhookResourceLinks,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    /// The URL that this webhook is configured to `POST` events to.
    pub url : String,
    /// An optional description that was provided at the time the webhook was created.
    pub description : Option<String>,
    /// A shared secret key used to sign all webhook events sent to the configured webhook URL. This field is returned only once, upon the initial creation of the webhook. If lost, create a new webhook and delete this webhook.
    /// The webhook URL receives a request with a `X-Up-Authenticity-Signature` header, which is the SHA-256 HMAC of the entire raw request body signed using this `secretKey`. It is advised to compute and check this signature to verify the authenticity of requests sent to the webhook URL. See Handling webhook events for full details.
    pub secret_key : Option<String>,
    /// The date-time at which this webhook was created.
    pub created_at : String,
}

#[derive(Deserialize, Debug)]
pub struct Relationships {
    pub logs : Logs,
}

#[derive(Deserialize, Debug)]
pub struct Logs {
    pub links : Option<LogsLinks>,
}

#[derive(Deserialize, Debug)]
pub struct LogsLinks {
    /// The link to retrieve the related resource(s) in this relationship.
    pub related : String,
}

#[derive(Deserialize, Debug)]
pub struct WebhookResourceLinks {
    /// The canonical link to this resource within the API.
    #[serde(rename = "self")]
    pub this : String,
}

#[derive(Deserialize, Debug)]
pub struct ResponseLinks {
    /// The link to the previous page in the results. If this value is `None` there is no previous page.
    pub prev : Option<String>,
    /// The link to the next page in the results. If this value is `None` there is no next page.
    pub next : Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct PingWebhookResponse {
    /// The webhook event data sent to the subscribed webhook.
    pub data : WebhookEventResource,
}

#[derive(Deserialize, Debug)]
pub struct WebhookEventResource {
    /// The type of this resource: `webhook-events`
    pub r#type : String,
    /// The unique identifier for this event. This will remain constant across delivery retries.
    pub id : String,
    pub attributes : EventAttributes,
    pub relationships : EventRelationships,
}

#[derive(Deserialize, Debug)]
pub struct EventRelationships {
    pub webhook : Webhook,
    pub transaction : Option<Transaction>,
}

#[derive(Deserialize, Debug)]
pub struct Transaction {
    pub data : TransactionData,
    pub links : Option<TransactionLinks>,
}

#[derive(Deserialize, Debug)]
pub struct Webhook {
    pub data : WebhookData,
    pub links : Option<WebhookLinks>,
}

#[derive(Deserialize, Debug)]
pub struct WebhookData {
    /// The type of this resource: `webhooks`
    pub r#type : String,
    /// The unique identifier of the resource within its type.
    pub id : String,
}

#[derive(Deserialize, Debug)]
pub struct WebhookLinks {
    /// The link to retrieve the related resource(s) in this relationship.
    pub related : String,
}

#[derive(Deserialize, Debug)]
pub struct TransactionData {
    /// The type of this resource: `transactions`
    pub r#type : String,
    /// The unique identifier of the resource within its type.
    pub id : String,
}

#[derive(Deserialize, Debug)]
pub struct TransactionLinks {
    /// The link to retrieve the related resource(s) in this relationship.
    pub related : String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EventAttributes {
    /// The type of this event. This can be used to determine what action to take in response to the event.
    pub event_type : standard::WebhookEventTypeEnum,
    /// The date-time at which this event was generated.
    pub created_at : String,
}


#[derive(Deserialize, Debug)]
pub struct ListWebhookLogsResponse {
    /// The list of delivery logs returned in this response.
    pub data : Vec<WebhookDeliveryLogResource>,
    pub links : LogsResponseLinks,
}

#[derive(Deserialize, Debug)]
pub struct WebhookDeliveryLogResource {
    /// The type of this resource: `webhook-delivery-logs`
    pub r#type : String,
    /// The unique identifier for this log entry.
    pub id : String,
    pub attributes : DeliveryLogAttributes,
    pub relationships : DeliveryLogRelationships,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DeliveryLogRelationships {
    pub webhook_event : WebhookEvent,
}

#[derive(Deserialize, Debug)]
pub struct WebhookEvent {
    pub data : WebhookEventData
}

#[derive(Deserialize, Debug)]
pub struct WebhookEventData {
    /// The type of this resource: `webhook-events`
    pub r#type : String,
    /// The unique identifier of the resource within its type.
    pub id : String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DeliveryLogAttributes {
    /// Information about the request that was sent to the webhook URL.
    pub request : Request,
    /// Information about the response that was received from the webhook URL.
    pub response : Option<Response>,
    /// The success or failure status of this delivery attempt.
    pub delivery_status : standard::WebhookDeliveryStatusEnum,
    /// The date-time at which this log entry was created.
    pub created_at : String,
}

#[derive(Deserialize, Debug)]
pub struct Request {
    /// The payload that was sent in the request body.
    pub body : String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    /// The HTTP status code received in the response.
    pub status_code : i64,
    /// The payload that was received in the response body.
    pub body : String,
}

#[derive(Deserialize, Debug)]
pub struct LogsResponseLinks {
    /// The link to the previous page in the results. If this value is `None` there is no previous page.
    pub prev : Option<String>,
    /// The link to the next page in the results. If this value is `None` there is no next page.
    pub next : Option<String>,
}


// ----------------- Input Objects -----------------

#[derive(Default)]
pub struct ListWebhooksOptions {
    /// The number of records to return in each page. 
    page_size : Option<u8>,
}

impl ListWebhooksOptions {
    /// Sets the page size.
    pub fn page_size(&mut self, value : u8) {
        self.page_size = Some(value);
    }

    fn add_params(&self, url : &mut reqwest::Url) {
        let mut query = String::new();

        if let Some(value) = &self.page_size {
            if !query.is_empty() {
                query.push('&');
            }
            query.push_str(&format!("page[size]={}", value));
        }

        if !query.is_empty() {
            url.set_query(Some(&query));
        }
    }
}

#[derive(Default)]
pub struct ListWebhookLogsOptions {
    /// The number of records to return in each page. 
    page_size : Option<u8>,
}

impl ListWebhookLogsOptions {
    /// Sets the page size.
    pub fn page_size(&mut self, value : u8) {
        self.page_size = Some(value);
    }

    fn add_params(&self, url : &mut reqwest::Url) {
        let mut query = String::new();

        if let Some(value) = &self.page_size {
            if !query.is_empty() {
                query.push('&');
            }
            query.push_str(&format!("page[size]={}", value));
        }

        if !query.is_empty() {
            url.set_query(Some(&query));
        }
    }
}

// ----------------- Request Objects -----------------

#[derive(Serialize)]
pub struct CreateWebhookRequest {
    /// The webhook resource to create.
    pub data : WebhookInputResource,
}

#[derive(Serialize)]
pub struct WebhookInputResource {
    pub attributes : InputAttributes,
}

#[derive(Serialize)]
pub struct InputAttributes {
    /// The URL that this webhook should post events to. This must be a valid HTTP or HTTPS URL that does not exceed 300 characters in length.
    pub url : String,
    /// An optional description for this webhook, up to 64 characters in length.
    pub description : Option<String>,
}

impl Client {
    ///  Retrieve a list of configured webhooks. The returned list is paginated and can be scrolled by following the `next` and `prev` links where present. Results are ordered oldest first to newest last.
    pub async fn list_webhooks(&self, options : &ListWebhooksOptions) -> Result<ListWebhooksResponse, error::Error> {
        let mut url = reqwest::Url::parse(&format!("{}/webhooks", BASE_URL)).map_err(error::Error::UrlParse)?;
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
                let webhook_response : ListWebhooksResponse = serde_json::from_str(&body).map_err(error::Error::Json)?;

                Ok(webhook_response)
            },
            _ => {
                let body = res.text().await.map_err(error::Error::BodyRead)?;
                let error : error::ErrorResponse = serde_json::from_str(&body).map_err(error::Error::Json)?;

                Err(error::Error::Api(error))
            }
        }
    }

    /// Retrieve a specific webhook by providing its unique identifier.
    pub async fn get_webhook(&self, id : &str) -> Result<GetWebhookResponse, error::Error> {
        // This assertion is because without an ID the request is thought to be a request for
        // many webhooks, and therefore the error messages are very unclear.
        if id.is_empty() {
            panic!("The provided webhook ID must not be empty.");
        }

        let url = reqwest::Url::parse(&format!("{}/webhooks/{}", BASE_URL, id)).map_err(error::Error::UrlParse)?;

        let res = reqwest::Client::new()
            .get(url)
            .header("Authorization", self.auth_header())
            .send()
            .await
            .map_err(error::Error::Request)?;

        match res.status() {
            reqwest::StatusCode::OK => {
                let body = res.text().await.map_err(error::Error::BodyRead)?;
                let webhook_response : GetWebhookResponse = serde_json::from_str(&body).map_err(error::Error::Json)?;

                Ok(webhook_response)
            },
            _ => {
                let body = res.text().await.map_err(error::Error::BodyRead)?;
                let error : error::ErrorResponse = serde_json::from_str(&body).map_err(error::Error::Json)?;

                Err(error::Error::Api(error))
            }
        }
    }

    /// Create a new webhook with a given URL. The URL will receive webhook events as JSON-encoded `POST` requests. The URL must respond with a HTTP `200` status on success.
    /// There is currently a limit of 10 webhooks at any given time. Once this limit is reached, existing webhooks will need to be deleted before new webhooks can be created.
    /// Event delivery is retried with exponential backoff if the URL is unreachable or it does not respond with a `200` status. The response includes a `secretKey` attribute, which is used to sign requests sent to the webhook URL. It will not be returned from any other endpoints within the Up API. If the `secretKey` is lost, simply create a new webhook with the same URL, capture its `secretKey` and then delete the original webhook. See Handling webhook events for details on how to process webhook events.
    /// It is probably a good idea to test the webhook by sending it a `PING` event after creating it.
    pub async fn create_webhook(&self, webhook_url : &str, description : Option<String>) -> Result<CreateWebhookResponse, error::Error> {
        let url = reqwest::Url::parse(&format!("{}/webhooks", BASE_URL)).map_err(error::Error::UrlParse)?;

        let body = CreateWebhookRequest {
            data : WebhookInputResource {
                attributes : InputAttributes { url : String::from(webhook_url), description }
            }
        };

        let body = serde_json::to_string(&body).map_err(error::Error::Serialize)?;

        let res = reqwest::Client::new()
            .post(url)
            .header("Authorization", self.auth_header())
            .header("Content-Type", "application/json")
            .body(body)
            .send()
            .await
            .map_err(error::Error::Request)?;

        match res.status() {
            reqwest::StatusCode::CREATED => {
                let body = res.text().await.map_err(error::Error::BodyRead)?;
                let webhook_response : CreateWebhookResponse = serde_json::from_str(&body).map_err(error::Error::Json)?;

                Ok(webhook_response)
            },
            _ => {
                let body = res.text().await.map_err(error::Error::BodyRead)?;
                let error : error::ErrorResponse = serde_json::from_str(&body).map_err(error::Error::Json)?;

                Err(error::Error::Api(error))
            }
        }
    }

    /// Delete a specific webhook by providing its unique identifier. Once deleted, webhook events will no longer be sent to the configured URL.
    pub async fn delete_webhook(&self, id : &str) -> Result<(), error::Error> {
        let url = reqwest::Url::parse(&format!("{}/webhooks/{}", BASE_URL, id)).map_err(error::Error::UrlParse)?;

        let res = reqwest::Client::new()
            .delete(url)
            .header("Authorization", self.auth_header())
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

    /// Send a `PING` event to a webhook by providing its unique identifier. This is useful for testing and debugging purposes. The event is delivered asynchronously and its data is returned in the response to this request.
    pub async fn ping_webhook(&self, id : &str) -> Result<PingWebhookResponse, error::Error> {
        let url = reqwest::Url::parse(&format!("{}/webhooks/{}/ping", BASE_URL, id)).map_err(error::Error::UrlParse)?;

        let res = reqwest::Client::new()
            .post(url)
            .header("Authorization", self.auth_header())
            .header("Content-Type", "application/json")
            .header("Content-Length", "0")
            .send()
            .await
            .map_err(error::Error::Request)?;

        match res.status() {
            reqwest::StatusCode::CREATED => {
                let body = res.text().await.map_err(error::Error::BodyRead)?;
                let webhook_response : PingWebhookResponse = serde_json::from_str(&body).map_err(error::Error::Json)?;

                Ok(webhook_response)
            },
            _ => {
                let body = res.text().await.map_err(error::Error::BodyRead)?;
                let error : error::ErrorResponse = serde_json::from_str(&body).map_err(error::Error::Json)?;

                Err(error::Error::Api(error))
            }
        }
    }

    /// Retrieve a list of delivery logs for a webhook by providing its unique identifier. This is useful for analysis and debugging purposes. The returned list is paginated and can be scrolled by following the `next` and `prev` links where present. Results are ordered newest first to oldest last. Logs may be automatically purged after a period of time.
    pub async fn list_webhook_logs(&self, id : &str, options : &ListWebhookLogsOptions) -> Result<ListWebhookLogsResponse, error::Error> {
        let mut url = reqwest::Url::parse(&format!("{}/webhooks/{}/logs", BASE_URL, id)).map_err(error::Error::UrlParse)?;
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
                let webhook_response : ListWebhookLogsResponse = serde_json::from_str(&body).map_err(error::Error::Json)?;

                Ok(webhook_response)
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

implement_pagination_v1!(ListWebhooksResponse);
