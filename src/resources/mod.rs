use super::AppState;
use std::sync::Mutex;

pub mod group;
pub mod health;
pub mod user;

pub use group::init as init_group_ns;
pub use user::init as init_user_ns;

fn log_request(route: &'static str, connections: &Mutex<u32>) {
    let mut con = connections.lock().unwrap();
    *con += 1;
    println!("{}\n\tconnections: {}", route, con);
}
