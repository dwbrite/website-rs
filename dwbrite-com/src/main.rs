#![feature(proc_macro_hygiene, decl_macro)]

use common::*;

use chrono::{Local, Timelike};
use rocket::config::Value;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;
use serde::{Deserialize, Serialize};
use grass::Error;
use std::fs;
use std::path::Path;
use std::ffi::OsStr;
use structopt::StructOpt;

mod blog;

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(short, long, default_value = "41234")]
    port: u16,
}


#[get("/")]
fn home() -> Template {
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct Ctx {
        title: String,
    }
    let c = Ctx {
        title: "Devin W. Brite".to_string(),
    };
    Template::render("home", c)
}

fn compile_sass() -> std::io::Result<()> {
    fn recurse_dirs<P: AsRef<Path>>(path: P) -> std::io::Result<()> {
        for entry in fs::read_dir(path).unwrap() {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                recurse_dirs(path)?;
                continue;
            }

            let metadata = fs::metadata(&path)?;

            // ignore files with the wrong type
            match path.extension() {
                Some(s) if s == "scss" => {}
                _ => continue
            }

            // ignore partial sass files
            match path.file_name() {
                Some(s) if !s.to_str().unwrap().starts_with("_") => {}
                _ => continue
            }

            // read and export non-partial sass files
            let css = grass::from_path(path.to_str().unwrap(), &grass::Options::default()).unwrap();
            let mut out = path.clone();
            out.set_extension("css");
            std::fs::write(out, css);
        }
        Ok(())
    }

    recurse_dirs(env!("CARGO_MANIFEST_DIR"))
}

fn main() {
    use blog::routes::MountBlog;

    compile_sass().unwrap();

    let opt = Opt::from_args();

    let mut dev = rocket::config::Config::development();
    dev.address = "0.0.0.0".to_string();
    dev.port = opt.port;
    dev.extras.insert(
        "template_dir".to_string(),
        Value::String("./templates".to_string()),
    );

    let rocket = rocket::custom(dev);

    rocket
        .mount("/", routes![home])
        .mount(
            "/resources",
            StaticFiles::from("./resources"),
        )
        .mount(
            "/blog/media",
            StaticFiles::from("./blog/media"),
        )
        .mount(
            "/portfolio",
            StaticFiles::from("./portfolio"),
        )
        .mount(
            "/resume",
            StaticFiles::from("./resume"),
        )
        .mount_blog()
        .attach(Template::fairing())
        .launch();
}
