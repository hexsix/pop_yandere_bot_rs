use std::fmt;
use std::path::Path;

use anyhow::Error;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub core: Core,
    pub db: Database,
    pub telegram: Telegram,
    pub yandere: Yandere,
}

impl Config {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let configs = std::fs::read_to_string(path)?;
        let configs: Config = toml::from_str(&configs)?;
        Ok(configs)
    }
}

#[derive(Debug, Deserialize)]
pub struct Core {
    pub log_level: String,
    pub scheduler: String,
}

#[derive(Deserialize)]
pub struct Database {
    pub database_url: String,
    pub expire: usize,
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
    pub rss_url: String,
    pub score_threshold: i32,
    pub updated_resend: bool,
}

#[cfg(test)]
mod test {
    use super::Config;

    #[test]
    fn ok() {
        if let Ok(configs) = Config::new("configs.toml") {
            assert!(vec!["trace", "debug", "info", "warn", "error"]
                .contains(&configs.core.log_level.as_str()))
        }
    }
}
