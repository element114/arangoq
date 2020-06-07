#![forbid(unsafe_code)]
#![warn(clippy::pedantic)]
#![allow(clippy::used_underscore_binding)]

#[cfg(feature = "actors")]
pub mod actor;

pub mod arango_api;
pub mod arango_connection;
pub mod arango_response;
pub mod database;
pub mod test;

#[cfg(feature = "actors")]
pub use actor::*;

pub use arango_api::*;
pub use arango_connection::*;
pub use arango_response::*;
pub use database::*;

mod arango_impl;
mod arango_test;

pub use arangoq_derive::*;
pub(crate) use maplit::*;
