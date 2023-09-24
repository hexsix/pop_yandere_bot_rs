use std::env;

use teloxide::prelude::*;
extern crate pretty_env_logger;
#[macro_use]
extern crate log;

#[macro_use]
mod yandere;
mod bot;
mod config;

use crate::bot::*;
use crate::config::Config;
use once_cell::sync::Lazy;

static CONFIG: Lazy<Config> = Lazy::new(|| {
    let config_file = std::env::var("EXLOLI_CONFIG");
    let config_file = config_file.as_deref().unwrap_or("configs.toml");
    Config::new(config_file).expect("配置文件解析失败")
});

fn init_env(log_level: &str, teloxide_token: &str) {
    env::set_var("RUST_LOG", log_level);
    env::set_var("TELOXIDE_TOKEN", teloxide_token);
}

fn init() {
    init_env(&CONFIG.core.log_level, &CONFIG.telegram.token);
    pretty_env_logger::init();
    info!("configs: {:?}", CONFIG);
}

#[tokio::main]
async fn main() {
    init();

    let bot = Bot::from_env();

    if let Ok(body) = yandere::request(&CONFIG.yandere.rss_url).await {
        let posts = yandere::parse_pop_recent(&body);
        info!("{} posts in total", posts.len());
        for (i, post) in posts.iter().enumerate() {
            if post.get_id() != 1121914 {
                continue;
            }
            info!("{} of {} is now processing.", i + 1, posts.len());
            if post.score_filter(CONFIG.yandere.score_threshold) {
                info!("post({}) filtered because of low score", post.get_id());
                continue;
            }
            // todo: already sent
            if let Ok(parent_id) = post.get_parent() {
                if let Ok(parent) = yandere::Post::new(parent_id).await {
                    let children = parent.get_children().await;
                    send_media_group(
                        &bot,
                        CONFIG.telegram.channel_id.clone(),
                        children,
                    )
                    .await;
                }
            } else {
                send_message(&bot, CONFIG.telegram.channel_id.clone(), post)
                    .await;
            }
        }
    }
}
