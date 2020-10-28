


mod configuration;
mod error;
mod pg_conf;

pub const CONF_PATH: &str = "configuration/configuration.toml";

pub use pg_conf::PgConf;
pub use error::ConfigurationError;
pub use configuration::Configuration;

