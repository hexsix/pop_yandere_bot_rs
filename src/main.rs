extern crate pretty_env_logger;
#[macro_use]
extern crate log;

use log::LevelFilter;

#[macro_use]
mod config;
mod yandere;

fn init_configs() -> config::Config {
    let configs = std::fs::read_to_string("configs.toml").unwrap();
    let configs: config::Config = toml::from_str(&configs).unwrap();
    configs
}

fn init_logger(log_level: &str) {
    let mut wrong_log_level = false;
    pretty_env_logger::formatted_builder()
        .filter_level(match log_level {
            "trace" => LevelFilter::Trace,
            "debug" => LevelFilter::Debug,
            "info" => LevelFilter::Info,
            "warn" => LevelFilter::Warn,
            "error" => LevelFilter::Error,
            _ => {
                wrong_log_level = true;
                LevelFilter::Info
            }
        })
        .init();
    if wrong_log_level {
        warn!("Wrong log_level, please check your configs.toml.");
        info!("Set log_level to info as default.");
    }
}

fn init() -> config::Config {
    let configs = init_configs();
    init_logger(&configs.core.log_level);
    info!("configs: {:?}", configs);

    configs
}

#[tokio::main]
async fn main() {
    let configs = init();
    if let Ok(body) = yandere::request(&configs.yandere.rss_url).await {
        let posts = yandere::parse_pop_recent(&body);
        info!("{} posts in total", posts.len());
        for (i, post) in posts.iter().enumerate() {
            info!("{} of {} is now processing.", i, posts.len());
            if post.score_filter(configs.yandere.score_threshold) {
                info!("post({}) filtered because of low score", post.get_id());
                continue;
            }
            // todo: already sent
            if let Ok(parent_id) = post.get_parent() {
                if let Ok(parent) = yandere::Post::new(parent_id).await {
                    let children = parent.get_children().await;
                    // todo: send children
                }
            } else {
                // todo: send post
            }
        }
    }
}
