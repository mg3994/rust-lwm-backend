use std::env;

pub struct Config {
    pub host: String,
    pub port: u16,
    pub db_host: String,
    pub db_port: u16,
    pub db_name: String,
    pub db_user: String,
    pub db_pass: String,
}

impl Config {
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        dotenv::dotenv().ok();

        let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
        let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string())
            .parse()
            .map_err(|_| "PORT must be a number")?;
            
        let db_host = env::var("DB_HOST").map_err(|_| "DB_HOST must be set")?;
        let db_port = env::var("DB_PORT").map_err(|_| "DB_PORT must be set")?
            .parse()
            .map_err(|_| "DB_PORT must be a number")?;
            
        let db_name = env::var("DB_NAME").map_err(|_| "DB_NAME must be set")?;
        let db_user = env::var("DB_USERNAME").map_err(|_| "DB_USERNAME must be set")?;
        let db_pass = env::var("DB_PASSWORD").map_err(|_| "DB_PASSWORD must be set")?;

        Ok(Self {
            host,
            port,
            db_host,
            db_port,
            db_name,
            db_user,
            db_pass,
        })
    }
}
