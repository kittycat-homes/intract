use clap::crate_name;
use once_cell::sync::Lazy;
use rand::distributions::{Alphanumeric, DistString};
use serde::{Deserialize, Serialize};
use tracing_subscriber::filter::{Directive, LevelFilter};

#[cfg(not(test))]
use crate::cli::CLI;

/// global var to store config in,
/// if config needs to be read, ideally use this
pub static CONFIG: Lazy<AppConfig> = Lazy::new(|| {
    // try loading config if specified in cli args,
    // there are not cli args in a test, this is why we skip it
    #[cfg(not(test))]
    if let Some(config_path) = CLI.config.clone() {
        match confy::load_path(config_path) {
            Ok(v) => return v,
            Err(e) => tracing::error!(
                "failed to load config from path! trying to load regular... {}",
                e
            ),
        }
    }

    match confy::load(crate_name!(), Some("config")) {
        Ok(conf) => conf,
        Err(e) => {
            tracing::error!("failed to load config, using default: {}", e);
            AppConfig::default()
        }
    }
});

/**
 * config file for this app
 * uses [confy](https://github.com/rust-cli/confy)
 */
#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    pub database: Database,
    pub logging: Logging,
    pub server: Server,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            server: Server {
                name: String::from("intract"),
                socket: "[::]:3000".into(),
                url: "http://localhost:3000".into(),
                min_password_size: 8,
                argon2_pepper: Alphanumeric.sample_string(&mut rand::thread_rng(), 64),
            },
            database: Database {
                db_url: "postgres://intract:hunter2@localhost/intract".into(),
            },
            logging: Logging {
                loglevel: Loglevel::default(),
            },
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Database {
    /**
     * database url
     * only postgres is supported
     * the format is:
     * `postgres://username:password@localhost/db_name`
     * if these differ for your postgres setup, swap them out
     */
    pub db_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Server {
    /**
     * what socket to listen on
     * if none of these work the app crashes
     */
    pub socket: String,

    /**
     * the url this runs on,
     * `https://example.com`
     */
    pub url: String,

    /**
     * how long a users password needs to be
     * at minimum
     */
    pub min_password_size: u8,

    /**
     * pepper to use for passwords hashed with argon2
     * randomly generated with default config
     * make sure not to delete this or it will get auto-generated
     * on startup. that would invalidate all your passwords
     */
    pub argon2_pepper: String,

    /**
     * what name to display for this instance
     */
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Logging {
    /**
     * how chatty this app should be while running
     * it is recommended to not use 'Debug' in production
     */
    pub loglevel: Loglevel,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, Default)]
pub enum Loglevel {
    Debug,
    Info,
    #[default]
    Warn,
    Error,
}

impl From<Loglevel> for Directive {
    fn from(value: Loglevel) -> Self {
        match value {
            Loglevel::Debug => LevelFilter::DEBUG,
            Loglevel::Error => LevelFilter::ERROR,
            Loglevel::Info => LevelFilter::INFO,
            Loglevel::Warn => LevelFilter::WARN,
        }
        .into()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /// try getting basic thing from config
    #[test]
    fn load_config() {
        assert!(CONFIG.server.min_password_size == AppConfig::default().server.min_password_size)
    }
}
