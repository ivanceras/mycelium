#![deny(warnings)]

pub use api::{Api, BaseApi};
pub use error::Error;
pub use types::metadata::Metadata;

// reexport dependencies crates
pub use codec;
pub use frame_metadata;
pub use sp_core;
pub use sp_runtime;
pub use sp_version;

mod api;
mod error;
pub mod types;
pub mod utils;
