use crate::config::Config;
use sqlx::{MySql, Pool};

pub type DbPool = Pool<MySql>;

pub async fn init(config: &Config) -> Result<DbPool, Box<dyn std::error::Error>> {
    println!("Initializing MySQL connection to {}...", config.db_name);
    
    // Construct connection string
    let url = if config.db_pass.is_empty() {
        format!(
            "mysql://{}@{}:{}/{}", 
            config.db_user, 
            config.db_host, 
            config.db_port, 
            config.db_name
        )
    } else {
        format!(
            "mysql://{}:{}@{}:{}/{}", 
            config.db_user, 
            config.db_pass, 
            config.db_host, 
            config.db_port, 
            config.db_name
        )
    };

    println!("Connecting to MySQL at {}:{}/{}", config.db_host, config.db_port, config.db_name);
    
    let pool = sqlx::MySqlPool::connect(&url).await?;
    
    println!("Successfully connected to MySQL!");

    Ok(pool)
}
