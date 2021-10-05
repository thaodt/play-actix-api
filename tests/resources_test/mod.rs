use super::{init_db_context, randomize_string};
use actix_web::web;
use actix_web::web::Data;
use play_actix_api::AppState;
use std::sync::{Arc, Mutex};

async fn init_app_state() -> Data<AppState<'static>> {
    let db_context = init_db_context().await;

    web::Data::new(AppState {
        connections: Mutex::new(0),
        context: Arc::new(db_context),
    })
}

mod group_test;
#[cfg(test)]
mod user_test;
