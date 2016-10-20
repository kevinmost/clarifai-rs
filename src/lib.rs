extern crate chrono;
#[macro_use] extern crate hyper;
extern crate serde;
extern crate serde_json;

pub mod prediction;
pub mod model;

use std::borrow::{
};
use std::cell::{
    Cell,
};
use std::io::{
    Read,
};
use std::marker::{
    Sized,
};
use std::str::{
    FromStr,
};

use hyper::client::{
    Client as HttpClient,
    Response as HttpResponse,
    Request as HttpRequest,
    RequestBuilder as HttpRequestBuilder,
};
use hyper::error::{
    Error as HttpError,
};
use hyper::status:: {
    StatusCode,
};
use hyper::header::{
    Authorization,
    Basic,
    Headers,
};

use serde_json::{
    Map,
    Value as JSON,
};
use serde_json::builder::{
    ObjectBuilder,
    ArrayBuilder,
};

use chrono::{
    DateTime,
    Duration,
    UTC,
};

use model:: {
    Model,
};
use prediction::{
    Prediction,
};

header! { (XClarifaiClient, "X-Clarifai-Client") => [String] }

// TODO: Make these &str's?
pub struct Client<'a> {
    pub client_id: String,
    pub client_secret: String,
    pub token: Cell<Option<Token<'a>>>,

    base_url: &'a str,
    http_client: Box<HttpClient>,
}

impl<'a> Client<'a> {
    pub fn new(id: &str, secret: &str) -> Client<'a> {
        let client_id =  String::from(id);
        let client_secret =  String::from(secret);
        let base_url =  "https://api2-dev.clarifai.com";
        let http_client = Box::new(HttpClient::new());

        let token: Response<Token>;
        {
            let refresh_token_request = Client::build_refresh_token_request(client_id.clone(), client_secret.clone(), base_url, *http_client);
            token = parse_response(refresh_token_request, |json| {
                let root = json.as_object().unwrap();
                Ok(Box::new(Token {
                    access_token: root.get("access_token").unwrap().as_str().unwrap(),
                    expires_at: UTC::now() + Duration::milliseconds(root.get("expires_in").unwrap().as_i64().unwrap()),
                }))
            });
        }

        return Client {
            client_id: client_id,
            client_secret: client_secret,
            token: Cell::new(None), // TODO

            base_url: base_url,
            http_client: http_client,
        };
    }

    // pub fn get_model_by_id(&self, id: &str) -> Response<Model> {
    //     let request = self.http_client
    //         .get(&format!("{}/models/{}", self.base_url, id))
    //     ;
    //     return parse_response(request, |json: JSONObject| Err("TODO"));
    // }

    // TODO: this is really limited. Use a Builder maybe
    pub fn predict_concepts(&self, model_id: &str, image_url: &str) -> Response<Vec<Prediction>> {
        let body = ObjectBuilder::new()
            .insert("inputs", ArrayBuilder::new()
                .push_object(|bld|{
                    bld.insert("data", ObjectBuilder::new()
                        .insert("image", ObjectBuilder::new()
                            .insert("url", &image_url)
                        .build())
                    .build())
                })
                .build()
            )
            .build().to_string();
        let request = self.http_client
            .post(&format!("{}/v2/models/{}/outputs", self.base_url, model_id))
            .body(&body);
        return parse_response(request, |json| {
            Err("TODO")
        });
    }

    fn build_refresh_token_request(
        client_id: String,
        client_secret: String,
        base_url: &str,
        http_client: HttpClient,
    ) -> HttpRequestBuilder {
        let basic_auth_header = Authorization(Basic{
            username: client_id,
            password: Some(client_secret),
        });
        return http_client.post(&format!("{}/v2/token", base_url))
        .body("\"grant_type\"=\"client_credentials\"")
        .header(basic_auth_header)
        ;
    }
}

#[derive(Clone, Copy)]
pub struct Token<'a> {
    pub access_token: &'a str,
    pub expires_at: DateTime<UTC>,
}

pub enum Response<'a, T: ?Sized> {
    NetworkError {
        http_error: HttpError,
    },
    ServerError {
        status_code: StatusCode,
    },
    ParsingError {
        status_code: StatusCode,
        error_details: &'a str,
    },
    Success {
        status_code: StatusCode,
        data: Box<T>,
    },
}

fn parse_response<'a, T: ?Sized, F>(request: HttpRequestBuilder, transform: F) -> Response<'a, T>
where F: Fn(JSON) -> Result<Box<T>, &'a str>
{
    let raw_result: Result<HttpResponse, HttpError> = request.send();
    if raw_result.is_err() {
        return Response::NetworkError{
            http_error: raw_result.unwrap_err(),
        };
    }

    let status_code: StatusCode;
    let json: JSON;
    {
        let mut raw_response = raw_result.unwrap();
        status_code = raw_response.status;

        if raw_response.status != hyper::Ok {
            return Response::ServerError {
                status_code: status_code,
            };
        }

        let mut raw_response_string = String::new();
        {
            raw_response.read_to_string(&mut raw_response_string);
        }
        json = serde_json::from_str(&raw_response_string).unwrap();
    }
    let transformed = transform(json);
    if transformed.is_err() {
        return Response::ParsingError {
            status_code: status_code,
            error_details: transformed.err().unwrap()
        };
    }
    return Response::Success {
        status_code: status_code,
        data: transformed.unwrap(),
    };
}
