pub mod config;
pub mod database;
pub mod error;
pub mod http;
pub mod mappers;
pub mod serialization;
pub mod utils;

pub static DATETIME_FORMAT: &str = "%Y-%m-%d %H:%M:%S %:z";
