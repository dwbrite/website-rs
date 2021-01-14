use chrono::NaiveDate;
use rocket::http::RawStr;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::sync::Arc;
use toml::value::Datetime;

pub(crate) mod routes {
    use crate::blog::*;
    use rocket::{Rocket, State};
    use rocket_contrib::templates::Template;

    pub(crate) trait MountBlog: Sized {
        fn mount_blog(self) -> Self;
    }

    impl MountBlog for Rocket {
        fn mount_blog(self) -> Self {
            self.mount("/", routes![blog, blog_post, blog_tags, blog_tag])
                .manage(BlogState::new())
        }
    }

    #[get("/blog")]
    fn blog(blogstate: State<BlogState>) -> Template {
        #[derive(Debug, Serialize, Deserialize, PartialEq)]
        struct Ctx {
            title: String,
            is_night: bool,
            posts: Vec<Arc<BlogPost>>,
        }

        let c = Ctx {
            title: "dwbrite.com".to_string(),
            is_night: crate::is_night(),
            posts: blogstate.sorted_posts.clone(),
        };

        Template::render("blog", &c)
    }

    #[get("/blog/post/<title>")]
    fn blog_post(title: &RawStr, state: State<BlogState>) -> Template {
        let key = title.url_decode().unwrap();
        let post = state.title_map.get(key.as_str()).unwrap();

        #[derive(Debug, Serialize, Deserialize, PartialEq)]
        struct Ctx {
            title: String,
            is_night: bool,
            posts: Vec<Arc<BlogPost>>,
        }

        let c = Ctx {
            title: "dwbrite.com".to_string(),
            is_night: crate::is_night(),
            posts: vec![post.clone()],
        };

        Template::render("blog", &c)
    }

    #[get("/blog/tags")]
    fn blog_tags() -> &'static str {
        "tags"
    }

    #[get("/blog/tags/<_name>")]
    fn blog_tag(_name: &RawStr) -> &'static str {
        "lol"
    }
}

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
            content: self.content,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct BlogPost {
    title: String,
    date: String,
    tags: Vec<String>,
    content: String,
}

/// TODO: real docs
struct BlogState {
    sorted_posts: Vec<Arc<BlogPost>>, // a list of posts sorted by date
    title_map: HashMap<String, Arc<BlogPost>>, // a map of posts with title as the key
    tags: HashMap<String, Vec<Arc<BlogPost>>>, // map of tags to posts
}

impl BlogState {
    fn new() -> Self {
        let mut s = Self {
            sorted_posts: vec![],
            title_map: Default::default(),
            tags: Default::default(),
        };

        s.load_posts();
        s.sort_posts();
        s
    }

    fn load_posts(&mut self) {
        // TODO: reset vecs/maps before filling?
        let posts_path = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/blog/posts"));

        for entry in fs::read_dir(posts_path).unwrap() {
            let entry_path = entry.unwrap().path();
            let file = File::open(entry_path).unwrap();
            let post = Arc::new(Self::read_post(file));
            self.sorted_posts.push(post.clone());
            self.title_map.insert(post.title.clone(), post.clone());

            for tag in &post.tags {
                if let Some(vec) = self.tags.get_mut(tag.as_str()) {
                    vec.push(post.clone());
                } else {
                    self.tags.insert(tag.clone(), vec![post.clone()]);
                }
            }
        }
    }

    fn sort_posts(&mut self) {
        self.sorted_posts.sort_by(|pa, pb| {
            let a = NaiveDate::parse_from_str(pa.date.as_str(), "%Y-%m-%d").unwrap();
            let b = NaiveDate::parse_from_str(pb.date.as_str(), "%Y-%m-%d").unwrap();

            b.cmp(&a)
        });
    }

    fn read_post(mut file: File) -> BlogPost {
        let mut s = String::new();
        file.read_to_string(&mut s).unwrap();
        // let mut buf_reader = BufReader::new(file);
        // TODO: toml won't read from ^--this buffer, so I'm converting to str first.

        let post: RawBlogPost = toml::from_str(s.as_str()).unwrap();
        post.into_blog_post()
    }
}
