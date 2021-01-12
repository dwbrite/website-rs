#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

// use crate::blog::routes::mount_blog;
use rocket::http::RawStr;
use rocket::Route;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;
use serde::{Deserialize, Serialize};

mod blog;

#[get("/")]
fn home() -> Template {
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct Ctx {
        title: String,
    }
    let c = Ctx {
        title: "dwbrite.com".to_string(),
    };
    Template::render("home", c)
}

fn main() {
    use blog::routes::MountBlog;

    let mut dev = rocket::config::Config::development();
    dev.address = "0.0.0.0".to_string();

    let mut rocket = rocket::custom(dev);
    // mount_blog(&mut rocket);

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
