use play_actix_api::config::Config;
use play_actix_api::repositories::Database;
use uuid::Uuid;

fn randomize_string(input: &'static str) -> String {
    format!("{0}{1}", input, Uuid::new_v4().to_string())
}

async fn init_db_context() -> Database<'static> {
    let config = Config::from_file("config.test.json");
    Database::new(&config.get_database_url()).await
}

#[cfg(test)]
mod resources_test;

