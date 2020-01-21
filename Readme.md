# libbookmarks

**WIP** library for storing url bookmarks in a sqlite3 database.
If no database path is given when creating the BookMarksApi struct, the
default of `$HOME/.local/share/libbookmarks/bookmarks.db` will be used.

### Example
```rust
   extern crate libbookmarks;

   use libbookmarks::*;

   fn main() -> Result<(), Error> {
     let api = BookMarksApi::new(None)?;
     println!("{:?}", api.all_bookmarks()?);
   }
```

### Features

- pyo3

  implements From<libbookmarks::Error> for Pyo3::PyErr
