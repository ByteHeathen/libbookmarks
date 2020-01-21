use diesel::Connection;
use diesel::SqliteConnection;
use diesel::QueryDsl;
use diesel::RunQueryDsl;

use crate::*;

use std::path::Path;
use std::path::PathBuf;

embed_migrations!("migrations/");

pub struct BookMarksApi {
    path: PathBuf,
    pub(crate) conn: SqliteConnection
}

impl BookMarksApi {

    pub fn new(input_path: Option<&Path>) -> Result<BookMarksApi, Error> {
        let path: PathBuf = if let Some(path) = input_path {
            path.to_owned()
        } else {
            default_file_path()?
        };
        match path.to_str() {
            Some(string) => {
                let conn = SqliteConnection::establish(string)?;
                embedded_migrations::run(&conn)?;
                Ok(BookMarksApi {
                    conn,
                    path
                })
            },
            None => {
                Err(Error::InvalidDatabasePath)
            }
        }
    }

    pub fn database_path(&self) -> PathBuf {
        self.path.clone()
    }

    pub fn all_tags(&self) -> Result<Vec<Tag>, Error> {
        use crate::schema::tags;

        Ok(tags::table.load(&self.conn)?)
    }

    pub fn get_tag(&self, tag: i32) -> Result<Tag, Error> {
        use crate::schema::tags;

        Ok(tags::table.find(tag).get_result(&self.conn)?)
    }

    pub fn create_tag(&self, tag: NewTag) -> Result<(), Error> {
        use crate::schema::tags;

        diesel::insert_into(tags::table)
            .values(&tag)
            .execute(&self.conn)?;
        Ok(())
    }

    pub fn all_folders(&self) -> Result<Vec<Folder>, Error> {
        use crate::schema::folders;

        Ok(folders::table.load(&self.conn)?)
    }

    pub fn get_folder(&self, id: i32) -> Result<Folder, Error> {
        use crate::schema::folders;

        Ok(folders::table.find(id).first(&self.conn)?)
    }

    pub fn create_folder(&self, folder: NewFolder) -> Result<(), Error> {
        use crate::schema::folders;

        diesel::insert_into(folders::table)
            .values(&folder)
            .execute(&self.conn)?;
        Ok(())
    }

    pub fn all_bookmarks(&self) -> Result<Vec<BookMark>, Error> {
        use crate::schema::bookmarks;

        Ok(bookmarks::table.load(&self.conn)?)
    }

    pub fn get_bookmark(&self, id: i32) -> Result<BookMark, Error> {
        use crate::schema::bookmarks;

        Ok(bookmarks::table.find(id).first(&self.conn)?)
    }

    pub fn create_bookmark(&self, bkm: NewBookMark) -> Result<(), Error> {
        use crate::schema::bookmarks;

        diesel::insert_into(bookmarks::table)
                .values(&bkm)
                .execute(&self.conn)?;
        Ok(())
    }
}

fn default_file_path() -> Result<PathBuf, Error> {
    use std::env::var;

    let mut path = PathBuf::from(var("HOME")?);
    path.push(".local/share/libbookmarks/bookmarks.db");
    Ok(path)
}
