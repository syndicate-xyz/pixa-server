use crate::config::load_env_config;
use std::sync::LazyLock;

pub mod config;
pub mod endpoints;
pub mod http;
pub mod math;
pub mod number;
pub mod solana;

// Cache env config to avoid loading it on every request
pub static ENV_CONFIG: LazyLock<crate::config::EnvConfig> =
    LazyLock::new(|| load_env_config().expect("Failed to load environment configuration"));
