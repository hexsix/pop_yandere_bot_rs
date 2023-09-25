use redis::{Commands, RedisResult};

use crate::yandere::Post;
use crate::{CONFIG, REDIS_CLIENT};

pub fn already_sent_post(post: &Post) -> RedisResult<bool> {
    let mut con = REDIS_CLIENT.get_connection()?;

    let key = &format!("id:{}", post.get_id());

    let result: Option<i64> = con.get(key)?;

    match result {
        Some(updated_at) => {
            if CONFIG.yandere.updated_resend
                && updated_at < post.get_updated_at()
            {
                debug!("resend post = {}", post.get_id());
                return Ok(false);
            }
            debug!("already sent post = {}", post.get_id());
            Ok(true)
        }
        None => {
            debug!("never send post = {}", post.get_id());
            Ok(false)
        }
    }
}

pub fn already_sent_posts(posts: &Vec<Post>) -> Result<bool, ()> {
    for post in posts {
        let result = already_sent_post(post);
        match result {
            Ok(result) => {
                if !result {
                    return Ok(false);
                }
            }
            Err(e) => {
                error!(
                    "error(query_redis), post = {}, error = {}",
                    post.get_id(),
                    e
                )
            }
        }
    }
    Ok(true)
}

pub fn set_redis_post(post: &Post) -> RedisResult<()> {
    let mut con = REDIS_CLIENT.get_connection()?;

    let key = &format!("id:{}", post.get_id());
    let value = post.get_updated_at();

    con.set_ex(key, value, CONFIG.db.expire)?;

    debug!("redis set key = {}, value = {}", key, value);
    Ok(())
}

pub fn set_redis_posts(posts: &Vec<Post>) -> Result<(), ()> {
    for post in posts {
        if let Err(e) = set_redis_post(post) {
            error!(
                "error(query_redis), post = {}, error = {}",
                post.get_id(),
                e
            );
            return Err(());
        }
    }
    Ok(())
}
