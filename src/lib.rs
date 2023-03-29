pub mod controllers;
pub mod errors;
pub mod models;
pub mod utils;

/// Modules needed by binaries
pub mod prelude {
    pub use crate::controllers::*;
}
