use redis::{Client, Commands, RedisResult};

use crate::yandere;
use yandere::Post;

pub fn already_sent_post(client: &Client, post: &Post) -> RedisResult<bool> {
    let mut con = client.get_connection()?;

    let key = &format!("id:{}", post.get_id());

    let result: Option<i64> = con.get(key)?;

    match result {
        Some(updated_at) => {
            if updated_at < post.get_updated_at() {
                debug!(
                    "ok, this post {} updated, send it again.",
                    post.get_id()
                );
                return Ok(false);
            }
            debug!("oh, this post {} has been sent already.", post.get_id());
            Ok(true)
        }
        None => {
            debug!("ok, this post {} hasn't been sent yet.", post.get_id());
            Ok(false)
        }
    }
}

pub fn already_sent_posts(
    client: &Client,
    posts: &Vec<Post>,
) -> RedisResult<bool> {
    for post in posts {
        let result = already_sent_post(client, post)?;
        if !result {
            return Ok(false);
        }
    }
    debug!("oh, all posts have been sent already.");
    Ok(true)
}

pub fn set_redis_post(
    client: &Client,
    post: &Post,
    expire: usize,
) -> RedisResult<()> {
    let mut con = client.get_connection()?;

    let key = &format!("id:{}", post.get_id());
    let value = post.get_updated_at();

    con.set_ex(key, value, expire)?;

    debug!("ok, set key = {}, value = {}", key, value);
    Ok(())
}

pub fn set_redis_posts(
    client: &Client,
    posts: &Vec<Post>,
    expire: usize,
) -> RedisResult<()> {
    for post in posts {
        set_redis_post(client, post, expire)?;
    }
    debug!("ok, set all posts.");
    Ok(())
}
