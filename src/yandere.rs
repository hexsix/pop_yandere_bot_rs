use std::fmt;

use lazy_static::lazy_static;
use quick_xml::events::Event;
use quick_xml::reader::Reader;
use regex::Regex;
use reqwest;
use serde_json;
use serde::Deserialize;

pub async fn request(url: &str) -> Result<String, reqwest::Error> {
    let body = reqwest::get(url).await?.text().await?;

    debug!("body = {:?}", body);

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
        debug!("post = {}", post);
        let post: Post = serde_json::from_str(post).expect("JSON was not well-formatted");
        debug!("post = {:?}", post);
        posts.push(post);
    }
    posts
}

#[derive(Debug, Deserialize)]
pub struct Post {
    id: i32,
    tags: String,
    created_at: i64,
    updated_at: i64,
    // author: String,
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
    children: Option<Vec<i32>>,
    parent_id: Option<i32>,
    is_held: bool,
}

#[derive(Debug, Clone)]
struct PostConstructError;

impl fmt::Display for PostConstructError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "cannot contruct Post")
    }
}

impl Post {
    async fn new(id: i32) -> Result<Self, PostConstructError> {
        let target = format!("https://yande.re/post.xml?tags=id:{}", id);
        if let Ok(xml) = request(&target).await {
            let mut reader = Reader::from_str(&xml);
            reader.trim_text(true);
            let mut buf = Vec::new();

            loop {
                match reader.read_event_into(&mut buf) {
                    Err(e) => warn!(
                        "Error at position {}: {:?}",
                        reader.buffer_position(),
                        e
                    ),
                    Ok(Event::Eof) => break,

                    Ok(Event::Start(ref e))
                        if e.name().as_ref() == b"post" =>
                    {
                        let mut tags = String::new();
                        let mut created_at = 0;
                        let mut updated_at = 0;
                        let mut source = String::new();
                        let mut score = 0;
                        let mut md5 = String::new();
                        let mut file_size = 0;
                        let mut file_ext = String::new();
                        let mut file_url = String::new();
                        let mut sample_url = String::new();
                        let mut sample_file_size = 0;
                        let mut rating = String::new();
                        let mut has_children = false;
                        let mut parent_id = None;
                        let mut is_held = false;
                        let children = vec![];
                        for attribute in e.attributes() {
                            if let Ok(attrib) = attribute {
                                match attrib.key.as_ref() {
                                    b"tags" => {
                                        tags = String::from(
                                            attrib
                                                .decode_and_unescape_value(
                                                    &reader,
                                                )
                                                .unwrap()
                                                .as_ref(),
                                        );
                                    }
                                    b"created_at" => {
                                        created_at = attrib
                                            .decode_and_unescape_value(&reader)
                                            .unwrap()
                                            .as_ref()
                                            .parse()
                                            .unwrap();
                                    }
                                    b"updated_at" => {
                                        updated_at = attrib
                                            .decode_and_unescape_value(&reader)
                                            .unwrap()
                                            .as_ref()
                                            .parse()
                                            .unwrap();
                                    }
                                    b"source" => {
                                        source = String::from(
                                            attrib
                                                .decode_and_unescape_value(
                                                    &reader,
                                                )
                                                .unwrap()
                                                .as_ref(),
                                        );
                                    }
                                    b"score" => {
                                        score = attrib
                                            .decode_and_unescape_value(&reader)
                                            .unwrap()
                                            .as_ref()
                                            .parse()
                                            .unwrap();
                                    }
                                    b"md5" => {
                                        md5 = String::from(
                                            attrib
                                                .decode_and_unescape_value(
                                                    &reader,
                                                )
                                                .unwrap()
                                                .as_ref(),
                                        );
                                    }
                                    b"file_size" => {
                                        file_size = attrib
                                            .decode_and_unescape_value(&reader)
                                            .unwrap()
                                            .as_ref()
                                            .parse()
                                            .unwrap();
                                    }
                                    b"file_ext" => {
                                        file_ext = String::from(
                                            attrib
                                                .decode_and_unescape_value(
                                                    &reader,
                                                )
                                                .unwrap()
                                                .as_ref(),
                                        );
                                    }
                                    b"file_url" => {
                                        file_url = String::from(
                                            attrib
                                                .decode_and_unescape_value(
                                                    &reader,
                                                )
                                                .unwrap()
                                                .as_ref(),
                                        );
                                    }
                                    b"sample_file_size" => {
                                        sample_file_size = attrib
                                            .decode_and_unescape_value(&reader)
                                            .unwrap()
                                            .as_ref()
                                            .parse()
                                            .unwrap();
                                    }
                                    b"sample_url" => {
                                        sample_url = String::from(
                                            attrib
                                                .decode_and_unescape_value(
                                                    &reader,
                                                )
                                                .unwrap()
                                                .as_ref(),
                                        );
                                    }
                                    b"rating" => {
                                        rating = String::from(
                                            attrib
                                                .decode_and_unescape_value(
                                                    &reader,
                                                )
                                                .unwrap()
                                                .as_ref(),
                                        );
                                    }
                                    b"has_children" => {
                                        has_children = attrib
                                            .decode_and_unescape_value(&reader)
                                            .unwrap()
                                            .as_ref()
                                            .parse()
                                            .unwrap();
                                    }
                                    b"parent_id" => {
                                        parent_id = match attrib
                                            .decode_and_unescape_value(&reader)
                                            .unwrap()
                                            .as_ref()
                                        {
                                            "" => None,
                                            s => Some(s.parse().unwrap()),
                                        };
                                    }
                                    b"is_held" => {
                                        is_held = attrib
                                            .decode_and_unescape_value(&reader)
                                            .unwrap()
                                            .as_ref()
                                            .parse()
                                            .unwrap();
                                    }
                                    _ => {}
                                }
                            }
                        }
                        return Ok(Self {
                            id,
                            tags,
                            created_at,
                            updated_at,
                            source,
                            score,
                            md5,
                            file_size,
                            file_ext,
                            file_url,
                            sample_url,
                            sample_file_size,
                            rating,
                            has_children,
                            children: Some(children),
                            parent_id,
                            is_held,
                        });
                    }

                    _ => (),
                }
                buf.clear();
            }
        }
        Err(PostConstructError)
    }

    pub fn parent(&self) {
        format!(
            "https://yande.re/post.xml?tags=parent:{}%20holds:true",
            self.id
        );
    }

    pub fn is_parent(&self) -> bool {
        self.parent_id.is_none()
    }
}
