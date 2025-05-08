use std::env;

#[derive(Debug)]
pub struct EnvConfig {
    pub port: u16,
    pub telegram_token: String,
    pub telegram_hook_url: String,
    pub vibe_api_key: String,
    pub database_url: String,
}

pub fn load_env_config() -> Result<EnvConfig, env::VarError> {
    let port = env::var("PORT")
        .unwrap_or("5000".to_string())
        .parse()
        .unwrap();
    let telegram_token = env::var("TELOXIDE_TOKEN")?;
    let telegram_hook_url = env::var("TELEGRAM_HOOK_URL")?;
    let vibe_api_key = env::var("VIBE_API_KEY")?;
    let database_url = env::var("DATABASE_URL")?;

    Ok(EnvConfig {
        port,
        telegram_token,
        telegram_hook_url,
        vibe_api_key,
        database_url,
    })
}
