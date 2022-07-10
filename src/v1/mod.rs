/// Error types and trait implementations.
pub mod error;
/// Types for modelling and interacting with [accounts](https://developer.up.com.au/#accounts).
pub mod accounts;
/// INCOMPLETE: Types for modelling and interacting with [categories](https://developer.up.com.au/#categories).
pub mod categories;
/// Types for modelling and interacting with [tags](https://developer.up.com.au/#tags).
pub mod tags;
/// INCOMPLETE: Types for modelling and interacting with [transactions](https://developer.up.com.au/#transactions).
pub mod transactions;
/// Types for modelling and interacting with [utilities](https://developer.up.com.au/#utility_endpoints).
pub mod utilities;
/// INCOMPLETE: Types for modelling and interacting with [webhooks](https://developer.up.com.au/#webhooks).
pub mod webhooks;

static BASE_URL : &str = "https://api.up.com.au/api/v1";

/// A client for interacting with the Up API.
pub struct Client {
    access_token : String,
}

impl Client {
    /// Creates an instance of the `Client` from the access token. Visit [this page](https://api.up.com.au/getting_started) to get such a token.
    pub fn new(access_token : String) -> Self {
        Client {
            access_token
        }
    }

    fn auth_header(&self) -> String {
        format!("Bearer {}", self.access_token)
    }
}
