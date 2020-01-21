use crate::BookMark;
use crate::BookMarksApi;
use crate::schema::tags;
use crate::schema::bookmark_tag_map;

use diesel::QueryDsl;
use diesel::RunQueryDsl;
use diesel::ExpressionMethods;
use crate::Error;


/// Tag used to organize bookmarks.
#[derive(Debug, Identifiable, Queryable, AsChangeset)]
#[table_name = "tags"]
pub struct Tag {
    /// Unique identifier for this tag.
    pub id: i32,
    /// Label used when displaying this tag.
    pub label: String,
    /// Color used when display this tag.
    pub color: Option<String>
}

impl Tag {

    /// Get a list of all the bookmarks belonging to this tag.
    pub fn bookmarks(&self, api: &BookMarksApi) -> Result<Vec<BookMark>, Error> {
        use crate::schema::bookmark_tag_map::dsl::*;
        use crate::schema::bookmarks;
        let mut out_bookmarks = vec![];
        for bkm_id in bookmark_tag_map.filter(tag_id.eq(self.id)).load::<BookMarkTagMap>(&api.conn)? {
            out_bookmarks.push(bookmarks::table.find(bkm_id.id).first(&api.conn)?);
        }
        Ok(out_bookmarks)
    }

    /// Save any changes to the fields of this tag.
    ///
    /// **NOTE:** Must be called after modifing either the
    /// `color` or the `label` paremeters.
    pub fn save(&self, api: &BookMarksApi) -> Result<(), Error> {
        use diesel::SaveChangesDsl;

        self.save_changes::<Tag>(&api.conn)?;
        Ok(())
    }
}

/// Create a new tag object
#[derive(Debug, Insertable)]
#[table_name = "tags"]
pub struct NewTag {
    /// The label used for displaying this tag.
    pub label: String,
    /// The color used when displaying this tag.
    pub color: Option<String>
}

#[derive(Debug, Identifiable, Queryable, Associations)]
#[table_name = "bookmark_tag_map"]
#[belongs_to(Tag, foreign_key = "id")]
#[belongs_to(BookMark, foreign_key = "id")]
pub(crate) struct BookMarkTagMap {
    pub id: i32,
    pub bookmark_id: i32,
    pub tag_id: i32
}

#[derive(Debug, Insertable)]
#[table_name = "bookmark_tag_map"]
pub(crate) struct NewBookMarkTagMap {
    pub bookmark_id: i32,
    pub tag_id: i32
}
