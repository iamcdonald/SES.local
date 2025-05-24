use std::sync::OnceLock;

use config::Config;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Conf {
    pub server: Server,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Server {
    pub port: u16,
    pub assets: Assets,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Assets {
    pub path: String,
}

impl Conf {
    pub fn get<'a>() -> &'a Self {
        static CONFIG: OnceLock<Conf> = OnceLock::new();
        CONFIG.get_or_init(|| {
            // parse and create Config
            Config::builder()
                .add_source(
                    config::Environment::with_prefix("APP")
                        .try_parsing(true)
                        .separator("_"),
                )
                .set_default("server.assets.path", "assets")
                .unwrap()
                .build()
                .unwrap()
                .try_deserialize()
                .unwrap()
        })
    }
}
