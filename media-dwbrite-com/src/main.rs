#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate rocket_multipart_form_data;
use rocket_multipart_form_data::{
    MultipartFormData, MultipartFormDataField, MultipartFormDataOptions,
};

use rocket::http::ContentType;
use rocket::response::NamedFile;
use rocket::Data;
use rocket_multipart_form_data::mime::Mime;
use std::fs::File;
use std::io::Write;
use std::path::Path;

#[get("/")]
fn index() -> NamedFile {
    let path = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/index.html"));
    NamedFile::open(path).unwrap()
}

#[post("/upload", data = "<data>")]
fn multipart_upload(content_type: &ContentType, data: Data) -> String {
    let options = MultipartFormDataOptions::with_multipart_form_data_fields(vec![
        MultipartFormDataField::raw("f").size_limit(10 * 1024 * 1024),
        MultipartFormDataField::text("desc"),
    ]);

    let mut multipart_form_data = MultipartFormData::parse(content_type, data, options).unwrap();

    let mut f = multipart_form_data.raw.remove("f").unwrap();
    let desc = multipart_form_data.texts.remove("desc").unwrap();
    let raw = f.remove(0);
    let mime = raw.content_type.unwrap();
    let name = raw.file_name.unwrap();
    let data = raw.raw;

    let mut file = File::create(format!("{}/media/{}", env!("CARGO_MANIFEST_DIR"), name)).unwrap();
    file.write_all(data.as_slice()).unwrap();

    match mime.to_string().as_str() {
        "image/gif" => {}
        "image/png" => {}
        "image/jpg" => {}
        "video/ogg" => {}
        "video/webm" => {}
        _ => {}
    }

    format!("{:?}, {:?}", mime, desc)
}

fn main() {
    let mut dev = rocket::config::Config::development();
    dev.address = "0.0.0.0".to_string();
    dev.port = 41233;

    rocket::custom(dev)
        .mount("/", routes![index, multipart_upload])
        .launch();
}
