use serde::{Deserialize, Serialize};
use toml::value::Datetime;

#[derive(Serialize, Deserialize, Clone)]
pub struct HtmlData {
    path: String, // url
    title: String,
    date: Datetime,
    tags: Vec<String>,
    content: String,
    hidden: Option<bool>,
}
