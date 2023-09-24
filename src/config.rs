use std::fmt;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub core: Core,
    pub db: Database,
    pub telegram: Telegram,
    pub yandere: Yandere,
}

#[derive(Debug, Deserialize)]
pub struct Core {
    pub log_level: String,
}

#[derive(Debug, Deserialize)]
pub struct Database {
    pub database_url: String,
}

#[derive(Deserialize)]
pub struct Telegram {
    pub token: String,
}

impl fmt::Debug for Telegram {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Telegram {{ token: ****** }}")
    }
}

#[derive(Debug, Deserialize)]
pub struct Yandere {
    pub rss_url: String,
    pub score_threshold: i32,
}

mod test {
    #[allow(unused_imports)]
    use super::Config;

    #[test]
    fn ok() {
        let configs = std::fs::read_to_string("configs.toml").unwrap();
        let configs: Config = toml::from_str(&configs).unwrap();
        let _ = configs;
    }
}
