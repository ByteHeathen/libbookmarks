use crate::BookMarksApi;
use crate::schema::tags;
use crate::schema::bookmark_tag_map;
use crate::models::BookMark;

use diesel::QueryDsl;
use diesel::RunQueryDsl;
use diesel::ExpressionMethods;
use diesel::result::Error;


#[derive(Debug, Identifiable, Queryable, AsChangeset)]
#[table_name = "tags"]
pub struct Tag {
    pub id: i32,
    pub label: String,
    pub color: Option<String>
}

impl Tag {

    pub fn bookmarks(&self, api: &BookMarksApi) -> Result<Vec<BookMark>, Error> {
        use crate::schema::bookmark_tag_map::dsl::*;
        use crate::schema::bookmarks;
        let mut out_bookmarks = vec![];
        for bkm_id in bookmark_tag_map.filter(tag_id.eq(self.id)).load::<BookMarkTagMap>(&api.conn)? {
            out_bookmarks.push(bookmarks::table.find(bkm_id.id).first(&api.conn)?);
        }
        Ok(out_bookmarks)
    }

    pub fn save(&self, api: &BookMarksApi) -> Result<(), Error> {
        use diesel::SaveChangesDsl;

        self.save_changes::<Tag>(&api.conn)?;
        Ok(())
    }
}

#[derive(Debug, Insertable)]
#[table_name = "tags"]
pub struct NewTag {
    pub label: String,
    pub color: Option<String>
}

#[derive(Debug, Identifiable, Queryable, Associations)]
#[table_name = "bookmark_tag_map"]
#[belongs_to(Tag, foreign_key = "id")]
#[belongs_to(BookMark, foreign_key = "id")]
pub struct BookMarkTagMap {
    pub id: i32,
    pub bookmark_id: i32,
    pub tag_id: i32
}

#[derive(Debug, Insertable)]
#[table_name = "bookmark_tag_map"]
pub struct NewBookMarkTagMap {
    pub bookmark_id: i32,
    pub tag_id: i32
}
