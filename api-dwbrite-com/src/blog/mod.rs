mod idx_fs_blog;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct BlogPostMetadata {
    id: String, // file location, database uid, whatever
    title: String,
    date: toml::value::Datetime,
    tags: Vec<String>,
    hidden: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct BlogPost {
    metadata: BlogPostMetadata,
    content: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct BlogPostContent {
    content: String
}

trait BlogPostProvider {
    fn list_tags() -> Vec<String>;
    fn list_posts() -> Vec<BlogPost>;
    fn list_posts_short() -> Vec<BlogPostMetadata>;
    fn list_posts_by_tag(tag: String) -> Vec<BlogPost>;
    fn get_post(id: String) -> BlogPost;
}
