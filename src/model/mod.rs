pub mod default;
pub mod request;

use serde_json;
use serde::de::{
    Visitor,
};
use chrono::{
    DateTime,
    UTC,
};

pub struct Version<'a> {
    pub id: &'a str,
    pub created_at: DateTime<UTC>,
    pub status: TrainingStatus,
}

pub enum TrainingStatus {
    NotYetTrained,
    NoPositiveExamples,
    TrainingQueued,
    TrainingInProgress,
    Trained,
}

pub struct ModelData<'a> {
    pub id: &'a str,
    pub name: Option<&'a str>,
    pub created_at: Option<DateTime<UTC>>,
    pub updated_at: Option<DateTime<UTC>>,
    pub app_id: Option<&'a str>,
    pub version: Option<Version<'a>>,
}

pub trait Model {}

pub struct ConceptModel<'a> {
    pub data: ModelData<'a>,
}
impl<'a> Model for ConceptModel<'a> {}

pub struct ColorModel<'a> {
    pub data: ModelData<'a>,
}
impl<'a> Model for ColorModel<'a> {}

struct ModelVisitor;
impl Visitor for ModelVisitor {
    type Value = Box<Model>;
}
