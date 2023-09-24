mod bot;
mod config;
mod db;
mod yandere;

use std::env;

use once_cell::sync::Lazy;
use redis::Client;
use teloxide::prelude::*;

use crate::bot::*;
use crate::config::Config;
use crate::db::*;
use crate::yandere::Post;

extern crate pretty_env_logger;
#[macro_use]
extern crate log;

static CONFIG: Lazy<Config> = Lazy::new(|| {
    Config::new("configs.toml").expect("Unable to parse configs.toml.")
});

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
    info!("configs: {:?}", CONFIG);
}

async fn run(bot: &Bot, post: &Post) {
    // filter score
    if post.score_filter(CONFIG.yandere.score_threshold) {
        info!("post({}) filtered because of low score", post.get_id());
        return;
    }
    // send message
    if let Ok(parent_id) = post.get_parent() {
        // send group
        if let Ok(parent) = yandere::Post::new(parent_id).await {
            let children = parent.get_children().await;
            let children_ids: Vec<i32> =
                children.iter().map(|m| m.get_id()).collect();
            match already_sent_posts(&REDIS_CLIENT, &children) {
                Ok(true) => {
                    info!(
                        "post({}) filtered because of already sent",
                        post.get_id()
                    );
                }
                Ok(false) => {
                    match send_media_group(
                        bot,
                        CONFIG.telegram.channel_id.clone(),
                        &children,
                    )
                    .await
                    {
                        Ok(_) => {
                            if set_redis_posts(
                                &REDIS_CLIENT,
                                &children,
                                CONFIG.db.expire,
                            )
                            .is_err()
                            {
                                warn!(
                                    "oh, redis set error, children = {:?}",
                                    children_ids
                                );
                            }
                        }
                        Err(_) => warn!(
                            "oh, telegram request error, children = {:?}",
                            children_ids
                        ),
                    }
                }
                Err(_) => {
                    warn!(
                        "oh, redis query error, children = {:?}",
                        children_ids
                    );
                }
            }
        }
    } else {
        // send single
        match already_sent_post(&REDIS_CLIENT, post) {
            Ok(true) => {
                info!(
                    "post({}) filtered because of already sent",
                    post.get_id()
                );
            }
            Ok(false) => {
                match send_message(
                    bot,
                    CONFIG.telegram.channel_id.clone(),
                    post,
                )
                .await
                {
                    Ok(_) => {
                        if set_redis_post(
                            &REDIS_CLIENT,
                            post,
                            CONFIG.db.expire,
                        )
                        .is_err()
                        {
                            warn!(
                                "oh, redis set error, post = {}",
                                post.get_id()
                            );
                        }
                    }
                    Err(_) => warn!(
                        "oh, telegram request error, post = {}",
                        post.get_id()
                    ),
                }
            }
            Err(_) => {
                warn!("oh, redis query error, post = {}", post.get_id());
            }
        }
    }
}

#[tokio::main]
async fn main() {
    init();

    let bot = Bot::from_env();

    if let Ok(body) = yandere::request(&CONFIG.yandere.rss_url).await {
        let posts = yandere::parse_pop_recent(&body);
        info!("{} posts in total", posts.len());
        for (i, post) in posts.iter().enumerate() {
            info!("{} of {} is now processing.", i + 1, posts.len());
            run(&bot, post).await;
        }
    }
}
