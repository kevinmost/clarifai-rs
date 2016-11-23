// stdlib
use std::borrow:: {
    Borrow,
};
use std::cell::{
    Cell,
};
use std::io:: {
    Read,
};
use std::sync::{
    Arc,
};

// libs
use chrono::{
    DateTime,
    Duration,
    UTC,
};
use hyper;
use hyper:: {
    Client as HttpClient,
};
use hyper::client::{
    Body as HttpBody,
    Response as HttpResponse,
    Request as HttpRequest,
    RequestBuilder as HttpRequestBuilder,
};
use hyper::error::{
    Error as HttpError,
};
use hyper::header:: {
    Authorization,
    Basic,
};
use hyper::method::{
    Method,
};
use hyper::status:: {
    StatusCode,
};
use serde_json;
use serde_json::{
    Map,
    Value as JSON,
};
use serde_json::error::{
    Error as JSONError,
};

// Clarifai
use request:: {
    Error,
    Request,
    Response,
};

header! { (XClarifaiClient, "X-Clarifai-Client") => [String] }

pub struct Client<'a, 't> {
    client_id: &'a str,
    client_secret: &'a str,
    token: Cell<Option<Token<'t>>>,

    pub base_url: &'a str,
    pub http_client: HttpClient,
}

impl<'a, 't> Client<'a, 't> {
    // TODO(Kevin): Maybe we don't want to return a Client, maybe we want to return some wrapper that will block if refresh(self) is currently happening?
    pub fn new(id: &'a str, secret: &'a str) -> Client<'a, 't> {
        let mut new = Client {
            client_id: id,
            client_secret: secret,
            token: Cell::new(None),

            base_url: "https://api2-dev.clarifai.com",
            http_client: HttpClient::new(),
        };
        new.refresh();
        return new;
    }

    /// Refresh takes a mutable reference so that nobody else can use this client while it's refreshing the token
    pub fn refresh(&mut self) {
        // TODO(Kevin): Some retrying logic here would be good... .unwrap() is definitely not safe
        let token = self.submit(&TokenRefreshRequest{}).unwrap();
        self.token.set(Some(*token));
    }

    pub fn post(&'a self, endpoint: &str) -> HttpRequestBuilder<'a> {
        return self.request(Method::Post, endpoint);
    }
    pub fn get(&'a self, endpoint: &str) -> HttpRequestBuilder<'a> {
        return self.request(Method::Get, endpoint);
    }
    // TODO(Kevin): other HTTP methods
    fn request(&'a self, method: Method, endpoint: &str) -> HttpRequestBuilder<'a> {
        return self.http_client.request(method, &format!("{}{}", self.base_url, endpoint));
    }

    // TODO(Kevin): Allow registering some 'static instance of the Client and then submitting a
    // request without a Client instance specified to use that default instance

    /// Submits any given Request to this Client, and returns the result of executing that request
    /// and unmarshaling it with the Request's specified unmarshaler
    pub fn submit<T>(&'a self, request: &Request<T>) -> Response<T> {
        // TODO(Kevin): Check if token is stale and refresh if so
        let mut http_response: HttpResponse = request.request(self).send()
            .map_err(|http_err| { Error::HTTP(http_err) })
        ?;

        let raw_response_string = read_to_string(&mut http_response);

        let status_code: StatusCode = http_response.status;
        if status_code != hyper::Ok {
            return Err(Error::API(status_code, raw_response_string));
        }

        let transformed: Result<Box<T>, JSONError>;
        {
            let json: JSON = serde_json::from_str(&raw_response_string).unwrap();
            transformed = request.unmarshal(&json);
        }
        return transformed.map_err(Error::JSON);
    }
}

#[derive(Clone, Copy)]
pub struct Token<'a> {
    pub access_token: &'a str,
    pub expires_at: DateTime<UTC>,
}

pub struct TokenRefreshRequest {}
impl<'a> Request<Token<'a>> for TokenRefreshRequest {
    fn request<'b>(&self, client: &'b Client) -> HttpRequestBuilder<'b> {
        let basic_auth_header = Authorization(Basic{
            username: client.client_id.to_owned(),
            password: Some(client.client_secret.to_owned()),
        });
        return client.post("/v2/token").header(basic_auth_header);
    }
    fn unmarshal(&self, json: &JSON) -> Result<Box<Token<'a>>, JSONError> {
        panic!("TODO");
    }
}

fn read_to_string(response: &mut HttpResponse) -> String {
    let mut string = String::new();
    {
        response.read_to_string(&mut string);
    }
    return string;
}
