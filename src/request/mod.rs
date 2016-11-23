pub mod client;

use std::io;
use std::sync::{
    Arc,
};
use hyper::client::{
    Client as HttpClient,
    RequestBuilder as HttpRequestBuilder,
};
use hyper::error::{
    Error as HttpError,
};
use hyper::header::{
    Header,
};
use hyper::status::{
    StatusCode,
};
use serde_json::{
    Value as JSON,
};
use serde_json::error::{
    Error as JSONError,
};
use request::client::{
    Client,
};

pub trait Request<T: ?Sized> {
    fn request<'a>(&self, client: &'a Client) -> HttpRequestBuilder<'a>;
    fn unmarshal(&self, json: &JSON) -> Result<Box<T>, JSONError>;
}

#[derive(Debug)]
pub enum Error {
    /// API returned an error status
    API(StatusCode, String),
    /// JSON was malformed
    JSON(JSONError),
    /// HTTP error occurred
    HTTP(HttpError),
}

pub type Response<T> = Result<Box<T>, Error>;
