use common::*;

use chrono::NaiveDate;
use rocket::http::RawStr;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::sync::Arc;
use toml::value::Datetime;
use common::regex::{Regex, Captures};
use common::media::{MediaData, MediaType};

pub(crate) mod routes {
    use crate::blog::*;
    use rocket::{Rocket, State};
    use rocket_contrib::templates::Template;

    pub(crate) trait MountBlog: Sized {
        fn mount_blog(self) -> Self;
    }

    impl MountBlog for Rocket {
        fn mount_blog(self) -> Self {
            self.mount(
                "/",
                routes![blog, blog_post, blog_tags, blog_tag, blog_live],
            )
            .manage(BlogState::new())
        }
    }

    #[get("/blog")]
    fn blog(blogstate: State<BlogState>) -> Template {
        #[derive(Debug, Serialize, Deserialize, PartialEq)]
        struct Ctx {
            title: String,
            posts: Vec<Arc<BlogPost>>,
        }

        let posts = blogstate
            .sorted_posts
            .iter()
            .filter_map(|p| {
                if p.hidden {
                    return None;
                }
                Some(p.clone())
            })
            .collect();

        let c = Ctx {
            title: "Devin's Blog".to_string(),
            posts,
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
            posts: Vec<Arc<BlogPost>>,
        }

        let c = Ctx {
            title: "Devin's Blog".to_string(),
            posts: vec![post.clone()],
        };

        Template::render("blog", &c)
    }

    #[get("/blog/live")]
    fn blog_live() -> Template {
        #[derive(Debug, Serialize, Deserialize, PartialEq)]
        struct Ctx {
            title: String,
            posts: Vec<Arc<BlogPost>>,
        }

        let blogstate = BlogState::new();

        let c = Ctx {
            title: "What?! You Shouldn't Be Here!".to_string(),
            posts: blogstate.sorted_posts.clone(),
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

pub fn transform_title(s: String) -> String {
    s.chars().filter_map(|c| -> Option<char> {
        return match c {
            '!' | '#' | '$' | '&' | '\'' | '(' | ')' | '*' | '/' | ':' | ';' | '=' | '?' | '@' | '[' | ']' => { None },
            '+' | ',' | ' ' => { Some('-') }
            _ => Some(c)
        }
    }).collect()
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct RawBlogPost {
    title: String,
    date: Datetime,
    tags: Vec<String>,
    content: String,
    hidden: Option<bool>,
}

impl RawBlogPost {
    fn into_blog_post(mut self) -> BlogPost {
        let hidden = match self.hidden {
            None => false,
            Some(b) => b,
        };

        self.codify_media();

        BlogPost {
            link_title: transform_title(self.title.clone()),
            title: self.title,
            date: self.date.to_string(),
            tags: self.tags,
            content: self.content,
            hidden,
        }
    }

    fn codify_media(&mut self) {
        let re = Regex::new(r#"<m src="(.*?)".*?>"#).unwrap();
        let result = re.replace_all(&self.content, |caps: &Captures| -> String {
            let mediadata_url = format!("http://media.dwbrite.com/registry/{}", &caps[1]);

            let mut option_data = None;
            match reqwest::blocking::get(&mediadata_url) {
                Ok(response) => {
                    println!("url: {}", mediadata_url);
                    println!("response? {:?}", response);
                    let bytes = response.bytes().unwrap();

                    let data: MediaData = serde_json::from_slice(bytes.to_vec().as_slice()).unwrap();
                    option_data = Some(data);
                }
                Err(e) => {
                    println!("REEEEEEEEEEEEEEE: {:?}", e);
                    return String::from("[Could not load media at startup. The media server may be down.]")
                }
            }

            let data = option_data.unwrap();

            use MediaType::*;
            match data.mediatype {
                PNG | JPEG | GIF => {
                    let mut class_list = vec!["media-content"];

                    if data.pixelated {
                        class_list.push("pxl");
                    }

                    let mut classes = String::new();
                    for s in class_list {
                        classes.push_str(s);
                        classes.push(' ');
                    }

                    let base = "https://media.dwbrite.com";
                    let src = format!("{}{}", base, data.file);
                    let thumb = format!("{}{}", base, data.thumbnail.unwrap());

                    format!(r#"
                        <noscript class="media-noscript" data-src="{src}">
                            <img class='{classes}' alt='{alt}' src='{src}' width='{width}' height='{height}'/>
                        </noscript>

                        <script>
                            store_img_data("{src}", {{
                                file: "{src}",
                                thumbnail: "{thumb}",
                                pixelated: {pixelated},
                                alt: "{alt}",
                                width: {width},
                                height: {height}
                            }});
                        </script>
                    "#, classes=classes, alt=data.alt, src=src, width=data.width, height=data.height, thumb=thumb, pixelated=data.pixelated)
                }
                BLOB => { String::from("{ media type not yet supported. contact dwbrite@gmail.com if you see this text. }") }
            }
        });
        self.content = result.to_string();
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct BlogPost {
    title: String,
    date: String,
    tags: Vec<String>,
    content: String,
    hidden: bool,
    link_title: String,
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
        let posts_path = concat!(env!("CARGO_MANIFEST_DIR"), "/blog/posts");

        for entry in fs::read_dir(posts_path).unwrap() {
            let entry_path = entry.unwrap().path();
            let file = File::open(entry_path).unwrap();

            let post = Arc::new(Self::read_post(file));
            self.sorted_posts.push(post.clone());
            self.title_map.insert(transform_title(post.title.clone()), post.clone());
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
