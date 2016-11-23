use hyper::client::{
    RequestBuilder as HttpRequestBuilder,
};
use serde_json::{
    Value as JSON,
};
use serde_json::error::{
    Error as JSONError,
};

// clarifai
use request::client::{
    Client,
};
use request::{
    Request,
};
use model::{
    Model,
};

struct GetModelRequest<'a> {
    id: &'a str,
    version_id: Option<&'a str>,
}
impl<'a> Request<Model> for GetModelRequest<'a> {
    fn request<'b>(&self, client: &'b Client) -> HttpRequestBuilder<'b> {
        return client.get(&format!("/v2/models/{}/output_info/{}",
            self.id,
            self.version_id.unwrap_or("")
        ));
    }

    fn unmarshal(&self, json: &JSON) -> Result<Box<Model>, JSONError> {
        json.find("")
    }
}
