use std::env;

#[derive(Debug)]
pub struct EnvConfig {
    pub port: u16,
    pub telegram_bot_api_token: String,
}

pub fn load_env_config() -> Result<EnvConfig, env::VarError> {
    let port = env::var("PORT")
        .unwrap_or("5000".to_string())
        .parse()
        .unwrap();
    let telegram_bot_api_token = env::var("TELEGRAM_BOT_API_TOKEN")?;

    Ok(EnvConfig {
        port,
        telegram_bot_api_token,
    })
}
