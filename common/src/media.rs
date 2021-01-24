use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum MediaType {
    PNG,
    JPEG,
    GIF,
    BLOB,
}

#[derive(Serialize, Deserialize)]
pub struct MediaData {
    url: String,
    thumbnail: Option<String>,
    mediatype: MediaType,
    pixelated: bool,
}

pub fn mime_to_mediatype(s: &str) -> MediaType {
    match s {
        "image/png" => MediaType::PNG,
        "image/jpeg" => MediaType::JPEG,
        "image/gif" => MediaType::GIF,
        _ => MediaType::BLOB,
    }
}
