#![allow(clippy::missing_const_for_fn)]
#![allow(clippy::module_name_repetitions)]
#![doc = include_str!("../README.md")]

pub use self::errors::Error;

pub mod prelude;

mod tera;

pub mod app;
pub mod controller;
pub mod errors;

pub mod validation;
pub use validator;

/// Application results options list
pub type Result<T, E = Error> = std::result::Result<T, E>;
