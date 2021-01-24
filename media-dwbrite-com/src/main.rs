#![feature(proc_macro_hygiene, decl_macro)]

use common::*;

use rocket_multipart_form_data::{
    MultipartFormData, MultipartFormDataField, MultipartFormDataOptions,
};

use image::imageops::FilterType;
use image::io::Reader as ImageReader;
use image::ImageFormat;
use rocket::http::ContentType;
use rocket::response::NamedFile;
use rocket::Data;
use rocket_multipart_form_data::mime::Mime;
use std::fs::File;
use std::io::{Cursor, Write};
use std::path::{Path, PathBuf};

#[get("/")]
fn homepage() -> NamedFile {
    let path = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/index.html"));
    NamedFile::open(path).unwrap()
}

#[post("/upload", data = "<data>")]
fn multipart_upload(content_type: &ContentType, data: Data) -> String {
    let options = MultipartFormDataOptions::with_multipart_form_data_fields(vec![
        MultipartFormDataField::raw("media").size_limit(10 * 1024 * 1024),
        MultipartFormDataField::text("description"),
        MultipartFormDataField::text("pixelated"),
    ]);

    let mut multipart_form_data = MultipartFormData::parse(content_type, data, options).unwrap();

    let mut f = multipart_form_data.raw.remove("media").unwrap();
    let desc = multipart_form_data.texts.remove("description").unwrap();
    let pixelated = multipart_form_data
        .texts
        .remove("pixelated")
        .unwrap()
        .remove(0)
        .text;
    println!("THIS IS PIXELATED: {}", pixelated);
    let raw = f.remove(0);
    let mime = raw.content_type.unwrap();
    let filename = raw.file_name.unwrap();
    let data = raw.raw;

    let thumbnail_result = save_thumbnail(&mime, &filename, &data);
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

    // TODO: save image metadata in a registry.json

    format!("{:?}, {:?}", mime, desc)
}

fn save_thumbnail(mime: &Mime, filename: &String, data: &Vec<u8>) -> Result<PathBuf, &'static str> {
    let format = match mime.to_string().as_str() {
        "image/gif" => Some(ImageFormat::Gif),
        "image/png" => Some(ImageFormat::Png),
        "image/jpeg" => Some(ImageFormat::Jpeg),
        _ => None,
    };

    if format.is_none() {
        // TODO: support non-image formats
        return Err("image format not supported");
    }

    let name = filename.split(".").next().unwrap();

    let path = Path::new(
        format!(
            "{}/media/thumbnail/{}-thumb.png",
            env!("CARGO_MANIFEST_DIR"),
            name
        )
        .as_str(),
    )
    .to_path_buf();

    // read the image with the proper format, then save it.
    ImageReader::with_format(Cursor::new(data), format.unwrap())
        .decode()
        .unwrap()
        .resize(32, 32, FilterType::Gaussian)
        .save_with_format(&path, ImageFormat::Png)
        .unwrap();

    Ok(path)
}

fn main() {
    let mut dev = rocket::config::Config::development();
    dev.address = "0.0.0.0".to_string();
    dev.port = 41233;

    // TODO: add rest api routes

    rocket::custom(dev)
        .mount("/", routes![homepage, multipart_upload])
        .launch();
}
