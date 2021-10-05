use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
struct AppConfig {
    url: String,
    port: u16,
}
#[derive(Deserialize)]
struct DbConfig {
    user: String,
    password: String,
    address: String,
    dbname: String,
}
#[derive(Deserialize)]
pub struct Config {
    app: AppConfig,
    conn: DbConfig,
}

impl Config {
    pub fn from_file(path: &'static str) -> Self {
        let config = fs::read_to_string(path).unwrap();
        serde_json::from_str(&config).unwrap()
    }

    pub fn get_app_url(&self) -> String {
        format!("{0}:{1}", self.app.url, self.app.port)
    }

    pub fn get_database_url(&self) -> String {
        format!(
            "mysql://{0}:{1}@{2}/{3}",
            self.conn.user, self.conn.password, self.conn.address, self.conn.dbname
        )
    }
}
