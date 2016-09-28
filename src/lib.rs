extern crate chrono;
extern crate hyper;

use std::cell::{
    Cell,
};

use hyper::client::{
    Client as HttpClient,
    Request as HttpRequest,
    Response as HttpResponse,
};

use chrono::{
    DateTime,
    UTC,
};

pub struct Client<'a> {
    pub client_id: String,
    pub client_secret: String,
    pub token: Cell<Option<Token<'a>>>,

    http_client: Box<HttpClient>,

    base_url: String,
}

impl<'a> Client<'a> {
    pub fn new(id: &str, secret: &str) -> Client<'a> {
        return Client {
            client_id: String::from(id),
            client_secret: String::from(secret),
            token: Cell::new(None),
            http_client: Box::new(HttpClient::new()),
            base_url: "https://api2-dev.clarifai.com/".to_string(),
        };
    }

    pub fn getModelByID<'b>(&self, id: &str) -> Result<'b, model::Model> {
        let url = self.base_url.clone() + "models/" + id;

        let response = self.http_client
            .get(&url)
            .send();

        // Parse response

        return Result {
            status: None,
            http_response: None,
            result: None,
        }
    }
}

#[derive(Clone, Copy)]
pub struct Token<'a> {
    pub access_token: &'a str,
    pub expires_at: DateTime<UTC>,
}

pub struct Result<'a, T> {
    pub status: Option<Status<'a>>,
    pub http_response: Option<HttpResponse>,
    pub result: Option<T>,
}
pub struct Status<'a> {
    pub code: i32,
    pub description: &'a str,
    pub details: &'a str,
}

pub mod prediction {
    use chrono::{
        DateTime,
        UTC,
    };

    pub trait Data {}
    pub struct ConceptData {
        id: Option<String>,
        name: Option<String>,
        created_at: Option<DateTime<UTC>>,
        updated_at: Option<DateTime<UTC>>,
        app_id: Option<String>,
        value: f32,
    }
    impl Data for ConceptData {}
    pub struct ColorData {

    }
    impl Data for ColorData {}

    pub enum Prediction {
        Concept(ConceptData),
        Color(ColorData),
    }
}

pub mod model {
    use chrono::{
        DateTime,
        UTC,
    };

    pub struct Version<'a> {
        id: &'a str,
        created_at: DateTime<UTC>,
        status: TrainingStatus,
    }

    pub enum TrainingStatus {
        NotYetTrained,
        NoPositiveExamples,
        TrainingQueued,
        TrainingInProgress,
        Trained,
    }

    pub trait Data {}
    pub struct ConceptData<'a> {
        id: &'a str,
        name: Option<String>,
        created_at: Option<DateTime<UTC>>,
        updated_at: Option<DateTime<UTC>>,
        app_id: Option<String>,
        version: Version<'a>,

    }
    impl<'a> Data for ConceptData<'a> {}
    pub struct ColorData {
    }
    impl Data for ColorData {}

    pub enum Model<'a> {
        Concept(ConceptData<'a>),
        Color(ColorData),
    }

}
