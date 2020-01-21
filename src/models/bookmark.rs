use crate::*;
use crate::schema::bookmarks;

use diesel::QueryDsl;
use diesel::RunQueryDsl;
use diesel::ExpressionMethods;
use diesel::result::Error;

#[derive(Debug, Identifiable, Queryable, AsChangeset)]
#[table_name = "bookmarks"]
pub struct BookMark {
    pub id: i32,
    pub url: String,
    pub label: Option<String>,
    pub folder: Option<i32>
}

impl BookMark {

    pub fn tags(&self, api: &BookMarksApi) -> Result<Vec<Tag>, Error> {
        use crate::schema::bookmark_tag_map::dsl::*;
        use crate::schema::tags;
        let mut out_tags = vec![];
        for tag in bookmark_tag_map.filter(bookmark_id.eq(self.id)).load::<BookMarkTagMap>(&api.conn)? {
            out_tags.push(tags::table.find(tag.id).first(&api.conn)?);
        }
        Ok(out_tags)
    }

    pub fn get_folder(&self, api: &BookMarksApi) -> Result<Option<Folder>, Error> {
        use crate::schema::folders::dsl::*;
        if let Some(folder) = self.folder {
            Ok(Some(folders.find(folder).first(&api.conn)?))
        } else {
            Ok(None)
        }
    }

    pub fn assign_tag(&self, api: &BookMarksApi, tag_id: i32) -> Result<(), Error> {
        use crate::schema::bookmark_tag_map;

        let new_tag_map = NewBookMarkTagMap {
            bookmark_id: self.id,
            tag_id
        };
        diesel::insert_into(bookmark_tag_map::table)
            .values(&new_tag_map)
            .execute(&api.conn)?;
        Ok(())
    }

    pub fn remove_tag(&self, api: &BookMarksApi, t_id: i32) -> Result<(), Error> {
        use crate::schema::bookmark_tag_map::dsl::*;
        let filter = bookmark_tag_map.filter(bookmark_id.eq(self.id))
                                     .filter(tag_id.eq(t_id));
        diesel::delete(filter).execute(&api.conn)?;
        Ok(())
    }

    pub fn save(&self, api: &BookMarksApi) -> Result<(), Error> {
        use diesel::SaveChangesDsl;

        self.save_changes::<BookMark>(&api.conn)?;
        Ok(())
    }
}

#[derive(Debug, Insertable)]
#[table_name = "bookmarks"]
pub struct NewBookMark {
    pub url: String,
    pub label: Option<String>,
    pub folder: Option<i32>
}
