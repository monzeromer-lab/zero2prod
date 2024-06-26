use config::{Config, File};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application_port: u16,
}

#[derive(Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

pub fn get_configration() -> Result<Settings, config::ConfigError>{
    let settings = Config::builder().add_source(File::with_name("configration")).build().expect("Faield to read configration.");
    settings.try_deserialize::<Settings>()
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> String {
        format!("postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database_name)
    }
    
    pub fn connection_string_without_db(&self) -> String {
        format!("postgres://{}:{}@{}:{}",
            self.username, self.password, self.host, self.port)
    }
}
