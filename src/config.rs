use dotenvy::dotenv;
use std::{env};

pub struct Config {
    // pub app_secret_key: String,
    // pub jwt_token_secret_key:String,
    // pub db_write_host:String,
    // pub db_read_host:String,
    // pub db_user:String,
    // pub db_password:String,
    // pub db_name:String
    pub db_url:String
}

pub fn config() -> Config {
    dotenv().ok();
    Config {
        // app_secret_key: env::var("APP_SECRET_KEY").expect("Env Read Error."),
        // jwt_token_secret_key:env::var("JWT_TOKEN_SECRET_KEY").expect("Env Read Error."),
        // db_write_host:env::var("DB_WRTIE_HOST").expect("Env Read Error."),
        // db_read_host:env::var("DB_READ_HOST").expect("Env Read Error."),
        // db_user:env::var("DB_USER").expect("Env Read Error."),
        // db_password:env::var("DB_PASSWORD").expect("Env Read Error."),
        // db_name:env::var("DB_NAME").expect("Env Read Error.")
        db_url:env::var("DATABASE_URL").expect("DATABASE_URL not set!")
        
    }
}
