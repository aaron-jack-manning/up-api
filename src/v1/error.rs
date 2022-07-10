use std::fmt;

use serde::Deserialize;

#[derive(Debug)]
/// Primary error type for requests made through `up_api::Client`.
pub enum Error {
    /// Represents cases where the URL could not be parsed correctly.
    UrlParse(url::ParseError),
    /// Represents an error in making the HTTP request.
    Request(reqwest::Error),
    /// Represents errors from the API (i.e. a non `2XX` response code).
    Api(ErrorResponse),
    /// Represents an error in deserializing JSON to the required structures. Occurances of this
    /// error should be treated as a bug in the library.
    Json(serde_json::Error),
    /// Represents an error in reading the body from the HTTP response. Occurances of this
    /// error should be treated as a bug in the library. 
    BodyRead(reqwest::Error),
    /// Represents an error serializing the data to be sent to the API. Occurances of this
    /// error should be treated as a bug in the library.
    Serialize(serde_json::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f : &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Self::UrlParse(val) => write!(f, "Failed to parse the URL before making the request: {:?}", val),
            Self::Request(val) => write!(f, "Failed to make the HTTP request to the API endpoint: {:?}", val),
            Self::Api(val) => write!(f, "The API returned an error response: {:?}", val),
            Self::Json(val) => write!(f, "Failed to deserialize the returned JSON to the correct format: {:?}", val),
            Self::BodyRead(val) => write!(f, "Failed to read the response body as a UTF-8 string: {:?}", val),
            Self::Serialize(val) => write!(f, "Failed to serialize the request data: {:?}", val),
        }
    }
}

impl std::error::Error for Error {}

#[derive(Deserialize, Debug)]
pub struct ErrorResponse {
    /// The list of errors returned in this response.
    pub errors : Vec<ErrorObject>,
}

#[derive(Deserialize, Debug)]
pub struct ErrorObject {
    /// The HTTP status code associated with this error. The status indicates the broad type of error according to HTTP semantics.
    pub status : String,
    /// A short description of this error. This should be stable across multiple occurrences of this type of error and typically expands on the reason for the status code.
    pub title : String,
    /// A detailed description of this error. This should be considered unique to individual occurrences of an error and subject to change. It is useful for debugging purposes.
    pub detail : String,
    /// If applicable, location in the request that this error relates to. This may be a parameter in the query string, or a an attribute in the request body.
    pub source : Option<Source>,
}

#[derive(Deserialize, Debug)]
pub struct Source {
    /// If this error relates to a query parameter, the name of the parameter.
    pub parameter : Option<String>,
    /// If this error relates to an attribute in the request body, a rfc-6901 JSON pointer to the attribute.
    pub pointer : Option<String>
}
