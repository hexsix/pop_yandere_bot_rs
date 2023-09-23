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
    pretty_env_logger::formatted_builder()
        .filter_level(match log_level {
            "debug" => LevelFilter::Debug,
            "info" => LevelFilter::Info,
            "warn" => LevelFilter::Warn,
            "error" => LevelFilter::Error,
            _ => {
                panic!("Wrong log_level, check your configs.toml.")
            }
        })
        .init();
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
        yandere::parse_pop_recent(&body);
    }
}
