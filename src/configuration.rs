use lazy_static::lazy_static;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Configuration {
    #[serde(default = "default_port")]
    pub port: u16,

    #[serde(default = "default_host")]
    pub host: String,

    pub shakespeare_token: Option<String>,
}

fn default_port() -> u16 {
    5000
}

fn default_host() -> String {
    "127.0.0.1".to_string()
}

lazy_static! {
    static ref CONFIGURATION: Configuration = {
        envy::prefixed("POKE_SPEARE_")
            .from_env::<Configuration>()
            .expect("wrong configuration from environment variables")
    };
}
pub fn get() -> &'static Configuration {
    &CONFIGURATION
}
