use actix_web::{web, App, HttpServer};
mod config;
use play_actix_api::repositories::Database;
use play_actix_api::{resources, AppState};
use std::sync::{Arc, Mutex};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("=== Playing Actix Web APIs ===");

    // Read in the configuration file
    let config_file: &'static str = "config.json";
    let config = config::Config::from_file(config_file);
    println!("Using configuration file from {0}", config_file);

    // Do connect to db
    let db_context = Database::new(&config.get_database_url()).await;
    println!("Connected to database: {0}", config.get_database_url());

    // Init the app_state - it will be cloned for each Actix thread
    // BUT the Arc of the DbContext will be reused in each Actix thread
    let app_state = web::Data::new(AppState {
        connections: Mutex::new(0),
        context: Arc::new(db_context),
    });

    let addr = &config.get_app_url();

    // Startup actix app
    let app = HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(web::scope("/health").route("", web::get().to(resources::health::health)))
            .configure(resources::init_group_ns)
            .configure(resources::init_user_ns)
    })
    .bind(addr)?;
    println!("Listening on: {0}", addr);

    app.run().await
}
