use super::models::{Group, User};

pub mod dbcontext;
mod group;
mod user;
mod user2group;

pub type Database<'c> = dbcontext::Database<'c>;
pub type Table<'c, T> = dbcontext::Table<'c, T>;
pub type JoinTable<'c, T1, T2> = dbcontext::JoinTable<'c, T1, T2>;