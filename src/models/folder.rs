use crate::schema::folders;
use crate::BookMark;
use crate::BookMarksApi;

use diesel::RunQueryDsl;
use crate::Error;
use diesel::ExpressionMethods;
use diesel::QueryDsl;

/// Container for a group of bookmarks.
#[derive(Debug, Identifiable, Queryable, Associations, AsChangeset)]
#[table_name = "folders"]
pub struct Folder {
    /// Unique identifier for this folder.
    pub id: i32,
    /// Label used when displaying this folder.
    pub label: String,
    /// Parent folder id.
    pub parent: Option<i32>
}

impl Folder {

    /// Get a list of all bookmarks belonging to this folder.
    pub fn bookmarks(&self, api: &BookMarksApi) -> Result<Vec<BookMark>, Error> {
        use crate::schema::bookmarks::dsl::*;
        let result = bookmarks.filter(folder.eq(&self.id));
        Ok(result.load(&api.conn)?)
    }

    /// Save changes made to the fields of this struct.
    ///
    /// **NOTE:** Must be called after modifing either the
    /// `parent` or the `label` paremeters.
    pub fn save(&self, api: &BookMarksApi) -> Result<(), Error> {
        use diesel::SaveChangesDsl;

        self.save_changes::<Folder>(&api.conn)?;
        Ok(())
    }
}

/// Create a new folder object.
#[derive(Debug, Insertable)]
#[table_name = "folders"]
pub struct NewFolder {
    /// The label to be used for display.
    pub label: String,
    /// The parent object of the folder.
    pub parent: Option<i32>
}
