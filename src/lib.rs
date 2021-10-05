use std::sync::{Arc, Mutex};
use repositories::Database;

pub mod config;
pub mod resources;
pub mod repositories;
pub mod models;

// AppState - our application's DI.
// Each test function that interacts with the db 
// will require an `AppState` instance to communicate with the database.
pub struct AppState<'a> {
    pub connections: Mutex<u32>,
    pub context: Arc<Database<'a>>,
}
