use crate::schema::folders;
use crate::BookMark;
use crate::BookMarksApi;

use diesel::RunQueryDsl;
use diesel::result::Error;
use diesel::ExpressionMethods;
use diesel::QueryDsl;

#[derive(Debug, Identifiable, Queryable, Associations, AsChangeset)]
#[table_name = "folders"]
pub struct Folder {
    pub id: i32,
    pub label: String,
    pub parent: Option<i32>
}

impl Folder {

    pub fn bookmarks(&self, api: &BookMarksApi) -> Result<Vec<BookMark>, Error> {
        use crate::schema::bookmarks::dsl::*;
        let result = bookmarks.filter(folder.eq(&self.id));
        Ok(result.load(&api.conn)?)
    }

    pub fn save(&self, api: &BookMarksApi) -> Result<(), Error> {
        use diesel::SaveChangesDsl;

        self.save_changes::<Folder>(&api.conn)?;
        Ok(())
    }
}

#[derive(Debug, Insertable)]
#[table_name = "folders"]
pub struct NewFolder {
    pub label: String,
    pub parent: Option<i32>
}
