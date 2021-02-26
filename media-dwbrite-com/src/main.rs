#![feature(proc_macro_hygiene, decl_macro)]

mod api;

use common::*;

use rocket_multipart_form_data::{
    MultipartFormData, MultipartFormDataField, MultipartFormDataOptions,
};

use crate::api::MountApi;
use common::media::MediaData;
use common::rocket_contrib::serve::StaticFiles;
use image::imageops::FilterType;
use image::io::Reader as ImageReader;
use image::ImageFormat;
use rocket::http::ContentType;
use rocket::response::NamedFile;
use rocket::Data;
use rocket_multipart_form_data::mime::Mime;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::{Cursor, Read, Write};
use std::path::Path;
use std::sync::Mutex;

#[get("/")]
fn homepage() -> NamedFile {
    let path = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/index.html"));
    NamedFile::open(path).unwrap()
}

#[post("/upload", data = "<data>")]
fn multipart_upload(
    registry: State<Mutex<MediaRegistry>>,
    content_type: &ContentType,
    data: Data,
) -> String {

    // TODO: add support for subdirs

    let options = MultipartFormDataOptions::with_multipart_form_data_fields(vec![
        MultipartFormDataField::raw("media").size_limit(10 * 1024 * 1024),
        MultipartFormDataField::text("description"),
        MultipartFormDataField::text("pixelated"),
    ]);

    let mut multipart_form_data = MultipartFormData::parse(content_type, data, options).unwrap();

    let mut f = multipart_form_data.raw.remove("media").unwrap();
    let desc = multipart_form_data
        .texts
        .remove("description")
        .unwrap()
        .first()
        .unwrap()
        .text
        .clone();
    let raw = f.remove(0);
    let mime = raw.content_type.unwrap();
    let filename = raw.file_name.unwrap();
    let data = raw.raw;

    let thumbnail = save_thumbnail(&mime, &filename, &data);
    // TODO: handle thumbnail result

    let mut file = File::create(format!(
        "{}/media/{}",
        env!("CARGO_MANIFEST_DIR"),
        filename.clone()
    ))
    .unwrap();
    file.write_all(data.as_slice()).unwrap();

    match mime.to_string().as_str() {
        "video/ogg" => {}
        "video/webm" => {}
        _ => {}
    }

    let mediadata = MediaData {
        file: format!("/media/{}", filename.clone()),
        thumbnail,
        mediatype: media::mime_to_mediatype(mime.to_string()),
        pixelated: is_pixelated(&mut multipart_form_data),
        alt: desc.clone(),
    };

    let mut r = registry.lock().unwrap();
    r.content.insert(filename.clone(), mediadata);
    r.save();

    format!("{:?}, {:?}", mime, desc)
}

fn save_thumbnail(mime: &Mime, filename: &String, data: &Vec<u8>) -> Option<String> {
    let format = match mime.to_string().as_str() {
        "image/gif" => Some(ImageFormat::Gif),
        "image/png" => Some(ImageFormat::Png),
        "image/jpeg" => Some(ImageFormat::Jpeg),
        _ => None,
    };

    if format.is_none() {
        // TODO: log error
        return None;
    }

    let thumbnail_location = format!("/media/thumb/{}-thumb.png", filename);

    let path = Path::new(format!("{}{}", env!("CARGO_MANIFEST_DIR"), thumbnail_location).as_str())
        .to_path_buf();

    // read the image with the proper format, then save it.
    ImageReader::with_format(Cursor::new(data), format.unwrap())
        .decode()
        .unwrap()
        .resize(32, 32, FilterType::Gaussian)
        .save_with_format(&path, ImageFormat::Png)
        .unwrap();

    Some(thumbnail_location)
}

fn is_pixelated(multipart_form_data: &mut MultipartFormData) -> bool {
    let pixelated = multipart_form_data
        .texts
        .remove("pixelated")
        .unwrap()
        .remove(0)
        .text;

    match pixelated.as_str() {
        "true" => true,
        _ => false,
    }
}

#[derive(Serialize, Deserialize)]
struct MediaRegistry {
    content: HashMap<String, MediaData>,
}

impl MediaRegistry {
    fn read_registry() -> MediaRegistry {
        let file = File::open(format!("{}/registry.toml", env!("CARGO_MANIFEST_DIR")).as_str());

        let mut registry = MediaRegistry {
            content: HashMap::new(),
        };

        if let Ok(mut f) = file {
            let mut s = String::new();
            f.read_to_string(&mut s).unwrap();
            // TODO: handle result
            registry = toml::from_str(s.as_str()).unwrap();
        }

        registry
    }

    fn file() -> File {
        let file_path = format!("{}/registry.toml", env!("CARGO_MANIFEST_DIR"));
        File::create(file_path.as_str()).unwrap()
    }

    fn save(&self) {
        let mut file = Self::file();
        let toml_data = toml::to_vec(self).expect("toml_string");

        file.write_all(toml_data.as_slice())
            .expect("failed to write toml");
    }
}

fn main() {
    let registry = MediaRegistry::read_registry();

    let mut dev = rocket::config::Config::development();
    dev.address = "0.0.0.0".to_string();
    dev.port = 41233;

    rocket::custom(dev)
        .mount("/", routes![homepage, multipart_upload])
        // TODO: serve static media files with cache headers and last-modified
        .mount(
            "/media",
            StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/media")),
        )
        .mount_api()
        .manage(Mutex::new(registry))
        .launch();
}
