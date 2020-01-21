//! # libbookmarks
//!
//! **WIP** library for storing url bookmarks in a sqlite3 database.
//! If no database path is given when creating the BookMarksApi struct, the
//! default of `$HOME/.local/share/libbookmarks/bookmarks.db` will be used.
//!
//! ### Example
//! ```rust
//!    extern crate libbookmarks;
//!
//!    use libbookmarks::*;
//!
//!    fn main() -> Result<(), Error> {
//!      let api = BookMarksApi::new(None)?;
//!      println!("{:?}", api.all_bookmarks()?);
//!      Ok(())
//!    }
//! ```

#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_migrations;

pub mod models;
pub use self::models::*;

pub mod schema;

mod error;
pub use self::error::Error;

mod api;
pub use self::api::BookMarksApi;
