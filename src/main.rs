#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket::config::Environment;
use rocket::Route;
use rocket::http::Method::Get;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufReader, Read};
use rocket::http::RawStr;
use rocket_contrib::templates::Template;
use rocket_contrib::serve::StaticFiles;
use rocket::logger::LoggingLevel;
use toml::value::Datetime;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct RawBlogPost {
    title: String,
    date: Datetime,
    tags: Vec<String>,
    content: String,
}

impl RawBlogPost {
    fn into_blog_post(self) -> BlogPost {
        BlogPost {
            title: self.title,
            date: self.date.to_string(),
            tags: self.tags,
            content: self.content
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct BlogPost {
    title: String,
    date: String,
    tags: Vec<String>,
    content: String,
}

#[get("/")]
fn home() -> Template {
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct ctx {
        title: String,
    }
    let c = ctx {
        title: "dwbrite.com".to_string()
    };
    Template::render("home", c)
}

#[get("/blog")]
fn blog() -> Template {
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct ctx {
        title: String,
        posts: Vec<BlogPost>,
    }
    let c = ctx {
        title: "dwbrite.com".to_string(),
        posts: vec![a()]
    };

    Template::render("blog", &c)
}

#[get("/blog/post/<title>")]
fn blog_post(title: &RawStr) -> &'static str {
    "o hej?"
}

#[get("/blog/tags")]
fn blog_tags() -> &'static str {
    "tags"
}

#[get("/blog/tags/<name>")]
fn blog_tag(name: &RawStr) -> &'static str {
    "lol"
}

fn main() {
    // let home = Route::new(Get, "/", home);
    // TODO: replace file routes with microservice, a.ohej.us
    // let files = Route::new(Get, "/files/<file..>", home);
    // let blog = Route::new(Get, "/blog", blog);
    a();

    let mut dev = rocket::config::Config::development();
    dev.address = "0.0.0.0".to_string();

    rocket::custom(dev)
        .mount("/", routes![home])
        .mount("/", routes![blog, blog_post, blog_tags, blog_tag])
        .mount("/resources", StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/resources")))
        .mount("/blog/media", StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/blog/media")))
        .attach(Template::fairing())
        .launch();
}

fn a() -> BlogPost {
    let mut file = File::open("blog/a.toml").unwrap();
    let mut s = String::new();
    file.read_to_string(&mut s).unwrap();
    // let mut buf_reader = BufReader::new(file);
    // TODO: toml won't read from ^--this buffer, so I'm converting to str first.

    let mut post: RawBlogPost = toml::from_str(s.as_str()).unwrap();
    post.into_blog_post()
}

// fn reload_blogposts() {
//     /*
//         let s = r#"
//     title = "pocket progress"
//     date = 2020-02-22
//     content = """
//     <noscript class="media-noscript">
//     """
//     "#;
//
//
//     */
//
//     // reference counted blogposts
//
//     // hashmap of blogposts using titles
//     // array of posts based on
//     // _map of vec of blogposts based on tags
//
//     // for each file in dir
//         // check if it's updated
//         // add it to the (???) template thing
// }