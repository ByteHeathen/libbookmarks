use diesel::Connection;
use diesel::SqliteConnection;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use diesel::ExpressionMethods;

use crate::*;

use std::path::PathBuf;

embed_migrations!("migrations/");

/// Api structure for interacting with
/// libbookmarks
pub struct BookMarksApi {
    path: PathBuf,
    pub(crate) conn: SqliteConnection
}

impl BookMarksApi {

    /// Create a new Api Object. If `input_path` is None
    /// `$HOME/.local/share/libbookmarks/bookmarks.db` is used.
    pub fn new(input_path: Option<String>) -> Result<BookMarksApi, Error> {
        let path = if let Some(path) = input_path {
            path
        } else {
            default_file_path()?
        };
        let conn = SqliteConnection::establish(&path)?;
        embedded_migrations::run(&conn)?;
        Ok(BookMarksApi {
            conn,
            path: PathBuf::from(path)
        })
    }

    /// Get the database path that is in use.
    pub fn database_path(&self) -> PathBuf {
        self.path.clone()
    }

    /// List all BookMark tags currently tracted by
    /// libbookmarks.
    pub fn all_tags(&self) -> Result<Vec<Tag>, Error> {
        use crate::schema::tags;

        Ok(tags::table.load(&self.conn)?)
    }

    /// Get a particular tag by id. Will return Error
    /// if not found.
    pub fn get_tag(&self, tag: i32) -> Result<Tag, Error> {
        use crate::schema::tags;

        Ok(tags::table.find(tag).get_result(&self.conn)?)
    }

    /// Remove a particular tag by id. Will return Error
    /// if not found.
    pub fn remove_tag(&self, id: i32) -> Result<(), Error> {
        use crate::schema::tags;
        diesel::delete(tags::table.find(id)).execute(&self.conn)?;
        Ok(())
    }

    /// Create a new tag from a `NewTag` object.
    pub fn create_tag(&self, tag: NewTag) -> Result<(), Error> {
        use crate::schema::tags;

        diesel::insert_into(tags::table)
            .values(&tag)
            .execute(&self.conn)?;
        Ok(())
    }

    /// List all folders tracted by libbookmarks.
    pub fn all_folders(&self) -> Result<Vec<Folder>, Error> {
        use crate::schema::folders;

        Ok(folders::table.load(&self.conn)?)
    }

    /// Get a particular folder by id. Will return Error
    /// if not found.
    pub fn get_folder(&self, id: i32) -> Result<Folder, Error> {
        use crate::schema::folders;

        Ok(folders::table.find(id).first(&self.conn)?)
    }

    /// Remove a particular folder by id. Will return Error
    /// if not found.
    pub fn remove_folder(&self, id: i32) -> Result<(), Error> {
        use crate::schema::folders;
        diesel::delete(folders::table.find(id)).execute(&self.conn)?;
        Ok(())
    }

    /// Create new folder from a `NewFolder` struct.
    pub fn create_folder(&self, folder: NewFolder) -> Result<(), Error> {
        use crate::schema::folders;

        diesel::insert_into(folders::table)
            .values(&folder)
            .execute(&self.conn)?;
        Ok(())
    }

    /// List all bookmarks currently tracted by
    /// libbookmarks.
    pub fn all_bookmarks(&self) -> Result<Vec<BookMark>, Error> {
        use crate::schema::bookmarks;

        Ok(bookmarks::table.load(&self.conn)?)
    }

    /// Get a particular bookmark by id. Will return Error
    /// if not found.
    pub fn get_bookmark(&self, id: i32) -> Result<BookMark, Error> {
        use crate::schema::bookmarks;

        Ok(bookmarks::table.find(id).first(&self.conn)?)
    }

    /// Remove a particular bookmark by id. Will return Error
    /// if not found.
    pub fn remove_bookmark(&self, id: i32) -> Result<(), Error> {
        use crate::schema::bookmarks;
        diesel::delete(bookmarks::table.find(id)).execute(&self.conn)?;
        Ok(())
    }

    /// Create a new bookmark from the `NewBookMark` struct.
    pub fn create_bookmark(&self, bkm: NewBookMark) -> Result<(), Error> {
        use crate::schema::bookmarks;

        diesel::insert_into(bookmarks::table)
                .values(&bkm)
                .execute(&self.conn)?;
        Ok(())
    }

    /// List all root folders.
    pub fn root_folders(&self) -> Result<Vec<Folder>, Error> {
        use crate::schema::folders::dsl::*;

        Ok(folders.filter(parent.eq::<Option<i32>>(None)).load(&self.conn)?)
    }
}

// Return the default file path used by libbookmarks.
// This will create the file and parent directory
// if none exists.
fn default_file_path() -> Result<String, Error> {
    use std::env::var;

    let mut path = PathBuf::from(var("HOME")?);
    path.push(".local/share/libbookmarks");
    if !path.exists() {
        std::fs::create_dir(&path)?;
    }
    path.push("bookmarks.db");
    if !path.exists() {
        std::fs::File::create(&path)?;
    }
    Ok(path.to_str().unwrap().to_owned())
}
