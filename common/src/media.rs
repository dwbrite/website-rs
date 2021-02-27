use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub enum MediaType {
    PNG,
    JPEG,
    GIF,
    BLOB,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct MediaData {
    pub file: String,
    pub thumbnail: Option<String>,
    pub mediatype: MediaType,
    pub pixelated: bool,
    pub alt: String,
    pub width: u32,
    pub height: u32,
}

pub fn mime_to_mediatype(s: String) -> MediaType {
    match s.as_str() {
        "image/png" => MediaType::PNG,
        "image/jpeg" => MediaType::JPEG,
        "image/gif" => MediaType::GIF,
        _ => MediaType::BLOB,
    }
}
