use lazy_static::lazy_static;
use regex::Regex;
use reqwest;
use serde::Deserialize;
use serde_json;
use serde_json::Value;

#[derive(Debug, Deserialize)]
pub struct Post {
    id: i32,
    tags: String,
    created_at: i64,
    updated_at: i64,
    source: String,
    score: i32,
    md5: String,
    file_size: i32,
    file_ext: String,
    file_url: String,
    sample_url: String,
    sample_file_size: i32,
    rating: String,
    has_children: bool,
    parent_id: Option<i32>,
    is_held: bool,
}

pub async fn request(url: &str) -> Result<String, reqwest::Error> {
    let body = reqwest::get(url).await?.text().await?;

    debug!("ok, request {}", url);
    trace!("body = {:?}", body);

    Ok(body)
}

fn extract_post(html: &str) -> Vec<&str> {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"Post\.register\((?P<json>\{.*?\})\)").unwrap();
    }
    let posts = RE.captures_iter(html).map(|c| c.name("json").unwrap());
    posts.map(|m| m.as_str()).collect()
}

pub fn parse_pop_recent(html: &str) -> Vec<Post> {
    let mut posts = vec![];
    for post in extract_post(html) {
        let post: Post = serde_json::from_str(post).expect("todo");
        trace!("post = {:?}", post);
        posts.push(post);
    }
    let post_ids: Vec<i32> = posts.iter().map(|m| m.id).collect();
    debug!("post_ids = {:?}", post_ids);
    posts
}

impl Post {
    pub async fn new(id: i32) -> Result<Post, ()> {
        let target =
            format!("https://yande.re/post.json?api_version=2&tags=id:{}", id);
        if let Ok(response) = request(&target).await {
            let post: Result<Value, _> = serde_json::from_str(&response);
            if let Ok(post) = post {
                let post = post.get("posts").unwrap().get(0).unwrap().clone();
                if let Ok(post) = serde_json::from_value(post) {
                    debug!("ok, new post {}", id);
                    trace!("post = {:?}", post);
                    return Ok(post);
                }
            }
        }
        Err(())
    }

    pub async fn get_children(&self) -> Vec<Post> {
        let mut children: Vec<Post> = vec![];
        let target = format!(
            "https://yande.re/post.json?api_version=2&tags=parent:{}%20holds:true",
            self.id
        );
        if let Ok(response) = request(&target).await {
            let posts: Result<Value, _> = serde_json::from_str(&response);
            if let Ok(posts) = posts {
                if let Some(posts) = posts.get("posts").unwrap().as_array() {
                    for post in posts {
                        if let Ok(post) = serde_json::from_value(post.clone())
                        {
                            children.push(post);
                        }
                    }
                }
            }
        }
        let target = format!(
            "https://yande.re/post.json?api_version=2&tags=parent:{}",
            self.id
        );
        if let Ok(response) = request(&target).await {
            let posts: Result<Value, _> = serde_json::from_str(&response);
            if let Ok(posts) = posts {
                if let Some(posts) = posts.get("posts").unwrap().as_array() {
                    for post in posts {
                        if let Ok(post) = serde_json::from_value(post.clone())
                        {
                            children.push(post);
                        }
                    }
                }
            }
        }
        let children_ids: Vec<i32> = children.iter().map(|m| m.id).collect();
        debug!("ok, children of {} = {:?}", self.id, children_ids);
        trace!("children = {:?}", children);
        children
    }

    pub fn get_id(&self) -> i32 {
        self.id
    }

    pub fn get_parent(&self) -> Result<i32, ()> {
        if let Some(parent_id) = self.parent_id {
            Ok(parent_id)
        } else {
            debug!("post of {} has no parent", self.id);
            Err(())
        }
    }

    pub fn score_filter(&self, score_threshold: i32) -> bool {
        debug!("post.score = {}, score_threshold = {}", self.score, score_threshold);
        self.score < score_threshold
    }
}

mod test {
    #[allow(unused_imports)]
    use super::Post;

    #[test]
    fn test_new_1121916() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        if let Ok(post) = rt.block_on(Post::new(1121916)) {
            assert_eq!(post.id, 1121916);
            assert_eq!(post.has_children, true);
            assert_eq!(post.created_at, 1695383691);
            assert_eq!(post.rating, "q");
            assert_eq!(post.get_parent(), Err(()));
            let children = rt.block_on(post.get_children());
            assert_eq!(children.len(), 2);
            if let Some(child) = children.get(0) {
                assert_eq!(child.id, 1121917);
            }
        }
    }
}
