mod client;
mod error;
mod organization;
mod team;
mod user;

pub type Result<T> = std::result::Result<T, error::Error>;
