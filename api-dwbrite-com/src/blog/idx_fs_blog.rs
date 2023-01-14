use std::ffi::OsStr;
use std::fs::File;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::{JoinHandle, Thread};
use serde::{Serialize, Deserialize};
use crate::blog::{BlogPost, BlogPostContent, BlogPostMetadata, BlogPostProvider};
use notify::{Watcher, RecommendedWatcher, RecursiveMode, Result, Event, EventKind};
use notify::event::{CreateKind, ModifyKind};
use notify::event::MetadataKind::Permissions;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct IndexedBlogPost {
    filename: String, // file location, database uid, whatever
    title: String,
    date: toml::value::Datetime,
    tags: Vec<String>,
    hidden: Option<bool>,
}

#[derive(Debug)]
struct IndexedFsBlog {
    root_dir: String,
    blog_posts: Arc<Mutex<Vec<BlogPost>>>,
    live_reload: bool,
}

impl IndexedFsBlog {
    pub fn new(root_dir: String, live_reload: bool) -> Self {
        let blog_posts = Arc::new(Mutex::new(Vec::new()));

        let mut blog = Self {
            root_dir,
            blog_posts,
            live_reload,
        };

        if live_reload {
            blog.configure_live_reload();
        }

        blog
    }

    fn configure_live_reload(&mut self) -> anyhow::Result<()> {
        let posts_clone = self.blog_posts.clone();
        let mut watcher = notify::recommended_watcher(move |res: Result<Event>| {
            let event = match res {
                Ok(e) => { e }
                Err(_) => {
                    todo!("print error");
                    return
                }
            };

            match event.kind {
                EventKind::Create(_) | EventKind::Modify(_) => {

                }
                EventKind::Remove(_) => {
                }
                _ => {
                    // log and ignore
                }
            };

            match res {
                Ok(event) if event.kind == EventKind::Create(CreateKind::Any) || event.kind == EventKind::Modify(ModifyKind::Any) => {
                    // let mut posts = posts_clone.lock().expect("Couldn't unlock this | TODO: deal with this error");

                    let paths: Vec<String> = event.paths.clone().iter().map(|buf| -> String {
                        String::from(buf.to_str().unwrap())
                    }).collect();

                    if paths.contains(&String::from("index.toml")) {
                        // create a new blog_posts data structure
                        let blog_posts = Vec::new();

                        // deserialize index.toml
                        let mut file = File::open("foo.txt").expect("file just changed, should exist??");
                        let xyz: Vec<IndexedBlogPost> = toml::from_slice()

                        // read every post/content
                        // build blog_posts
                        // write out blog_posts
                    }

                    posts.iter_mut().filter(|post| {

                    });

                    for p in event.paths {
                        match p.file_name() {
                            None => {}
                            Some(filename) if filename == "index.toml" => {
                                // find linked file
                                // update metadata
                            }
                            Some(filename) => {
                                // find a post with this name (if possible)
                                // update post
                                for post in posts.iter_mut(). {
                                    if post.metadata.id == filename.to_str().expect("Can't convert OsStr to str") {

                                    }
                                }
                            }
                        }
                    }
                },
                Err(e) => {
                    // TODO: log errors
                },
            }
        })?;

        watcher.watch(Path::new(&self.root_dir), RecursiveMode::NonRecursive).expect("Could not start watcher.");

        Ok(())
    }
}


impl BlogPostProvider for IndexedFsBlog {
    fn list_tags() -> Vec<String> {
        todo!()
    }

    fn list_posts() -> Vec<BlogPost> {
        todo!()
    }

    fn list_posts_short() -> Vec<BlogPostMetadata> {
        todo!()
    }

    fn list_posts_by_tag(tag: String) -> Vec<BlogPost> {
        todo!()
    }

    fn get_post(id: String) -> BlogPost {
        todo!()
    }
}