mod bot;
mod config;
mod db;
mod yandere;

use std::env;

use once_cell::sync::Lazy;
use redis::Client;
use teloxide::prelude::*;

use crate::bot::send_media_group;
use crate::config::Config;
use crate::db::*;
use crate::yandere::Post;

extern crate pretty_env_logger;
#[macro_use]
extern crate log;

static CONFIG: Lazy<Config> = Lazy::new(|| {
    Config::new("configs.toml").expect("Unable to parse configs.toml.")
});

static BOT: Lazy<Bot> = Lazy::new(Bot::from_env);

static REDIS_CLIENT: Lazy<Client> = Lazy::new(|| {
    Client::open(CONFIG.db.database_url.clone())
        .expect("Unable to open redis client.")
});

fn init_env(log_level: &str, teloxide_token: &str) {
    env::set_var("RUST_LOG", log_level);
    env::set_var("TELOXIDE_TOKEN", teloxide_token);
}

fn init() {
    init_env(&CONFIG.core.log_level, &CONFIG.telegram.token);
    pretty_env_logger::init();
    info!("ok(init config), configs = {:?}", CONFIG);
}

async fn run(post: &Post) {
    if post.score_filter(CONFIG.yandere.score_threshold) {
        info!("filtered(low score), post = {}", post.get_id());
        return;
    }
    let mut posts = vec![];
    if let Ok(parent_id) = post.get_parent() {
        if let Ok(parent) = yandere::Post::new(parent_id).await {
            posts = parent.get_children().await;
        }
    } else if post.has_children() {
        posts = post.get_children().await;
    }
    if posts.is_empty() {
        posts = vec![post.clone()];
    }
    if let Ok(false) = already_sent_posts(&posts) {
        if send_media_group(&posts).await.is_ok() {
            let _ = set_redis_posts(&posts);
        }
    }
}

#[tokio::main]
async fn main() {
    init();

    match yandere::request(&CONFIG.yandere.rss_url).await {
        Ok(body) => {
            let posts = yandere::parse_pop_recent(&body);
            info!("ok, {} posts in total", posts.len());
            for (i, post) in posts.iter().enumerate() {
                info!("ok, {} of {} is now processing.", i + 1, posts.len());
                run(post).await;
            }
        }
        Err(e) => error!("error(request yandere). error = {}", e),
    }
}
