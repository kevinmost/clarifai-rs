use chrono::{
    DateTime,
    UTC,
};

// TODO: Maybe make this into a "sealed class" like construct so that they can be statically typed too
pub trait Prediction {}

pub struct Concept<'a> {
    id: Option<&'a str>,
    name: Option<&'a str>,
    created_at: Option<DateTime<UTC>>,
    updated_at: Option<DateTime<UTC>>,
    app_id: Option<&'a str>,
    value: f32,
}
impl<'a> Prediction for Concept<'a> {}

pub struct Color<'a> {
    // TODO: Make the hex and webSafeHex into some RgbColor tuple with three u8's instead of just a raw string
    hex: &'a str,
    web_safe_hex: &'a str,
    web_safe_color_name: &'a str,
    value: f32,
}
impl<'a> Prediction for Color<'a> {}
