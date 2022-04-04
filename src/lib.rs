#[macro_use]
extern crate rocket;

#[macro_use]
pub(crate) mod macros;

/// All server related
pub mod server;

/// All guards/ssl generation/etc...
pub mod secure;

/// All the Routes/endpoints
mod controllers;

/// All models
pub mod models;

/// Database Backend
pub mod db;

/// App related Errors
pub mod error;
pub type Result<T> = std::result::Result<T, error::Error>;
