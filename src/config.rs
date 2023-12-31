use std::fmt;

use anyhow::Error;
use figment::{
    providers::{Env, Format, Toml},
    Figment,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub core: Core,
    pub db: Database,
    pub telegram: Telegram,
    #[serde(default)]
    pub yandere: Yandere,
}

impl Config {
    pub fn new() -> Result<Self, Error> {
        let configs: Config = Figment::new()
            .merge(Toml::file("configs.toml"))
            .merge(Env::prefixed("APP_"))
            .extract()?;
        Ok(configs)
    }
}

#[derive(Debug, Deserialize)]
pub struct Core {
    #[serde(default = "log_level_default")]
    pub log_level: String,
    #[serde(default = "scheduler_default")]
    pub scheduler: String,
}

fn log_level_default() -> String {
    String::from("info")
}

fn scheduler_default() -> String {
    String::from("0 0 0,9,12,15,18,21 * * *")
}

impl Default for Core {
    fn default() -> Self {
        Core { log_level: log_level_default(), scheduler: scheduler_default() }
    }
}

#[derive(Deserialize)]
pub struct Database {
    pub database_url: String,
    #[serde(default = "expire_default")]
    pub expire: usize,
}

fn expire_default() -> usize {
    7776000
}

impl fmt::Debug for Database {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Database {{ database_url: ******, expire: {} }}",
            self.expire
        )
    }
}

#[derive(Deserialize)]
pub struct Telegram {
    pub token: String,
    pub channel_id: String,
}

impl fmt::Debug for Telegram {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Telegram {{ token: ******, channel_id: {} }}",
            self.channel_id
        )
    }
}

#[derive(Debug, Deserialize)]
pub struct Yandere {
    #[serde(default = "rss_url_default")]
    pub rss_url: String,
    #[serde(default = "score_threshold_default")]
    pub score_threshold: i32,
    #[serde(default = "updated_resend_default")]
    pub updated_resend: bool,
}

fn rss_url_default() -> String {
    String::from("https://yande.re/post/popular_recent")
}

fn score_threshold_default() -> i32 {
    0
}

fn updated_resend_default() -> bool {
    false
}

impl Default for Yandere {
    fn default() -> Self {
        Yandere {
            rss_url: rss_url_default(),
            score_threshold: score_threshold_default(),
            updated_resend: updated_resend_default(),
        }
    }
}
