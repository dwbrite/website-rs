#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use chrono::{Local, Timelike};
use rocket::config::Value;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;
use serde::{Deserialize, Serialize};

mod blog;

#[get("/")]
fn home() -> Template {
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct Ctx {
        title: String,
        is_night: bool,
    }
    let c = Ctx {
        title: "dwbrite.com".to_string(),
        is_night: is_night(),
    };
    Template::render("home", c)
}

fn is_night() -> bool {
    let hour = Local::now().time().hour();
    hour > 19 || hour < 7
}

fn main() {
    use blog::routes::MountBlog;

    let mut dev = rocket::config::Config::development();
    dev.address = "0.0.0.0".to_string();
    dev.port = 41234;
    dev.extras.insert(
        "template_dir".to_string(),
        Value::String(concat!(env!("CARGO_MANIFEST_DIR"), "/templates").to_string()),
    );

    let rocket = rocket::custom(dev);

    rocket
        .mount("/", routes![home])
        .mount(
            "/resources",
            StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/resources")),
        )
        .mount(
            "/blog/media",
            StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/blog/media")),
        )
        .mount_blog()
        .attach(Template::fairing())
        .launch();
}
