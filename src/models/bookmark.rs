use crate::*;
use crate::schema::bookmarks;
use crate::models::tag::BookMarkTagMap;
use crate::models::tag::NewBookMarkTagMap;

use diesel::QueryDsl;
use diesel::RunQueryDsl;
use diesel::ExpressionMethods;
use crate::Error;

/// A BookMarked Url managed by libbookmarks.
#[derive(Debug, Identifiable, Queryable, AsChangeset)]
#[table_name = "bookmarks"]
pub struct BookMark {
    /// Unique identifier for this bookmark.
    pub id: i32,
    /// The url this bookmark links to.
    pub url: String,
    /// The label used when displaying this bookmark.
    pub label: Option<String>,
    /// The id of the folder this bookmark belongs to.
    pub folder: Option<i32>,
    /// Whether this BookMark is `starred` in the ui.
    pub starred: bool
}

impl BookMark {

    /// Return a bookmark with the given `url_req`.
    pub fn find_url(api: &BookMarksApi, url_req: &str) -> Result<Vec<BookMark>, Error> {
        use crate::schema::bookmarks::dsl::*;

        Ok(bookmarks.filter(url.eq(&url_req)).get_results(&api.conn)?)
    }

    /// Return any bookmarks with the given label.
    pub fn find_label(api: &BookMarksApi, label_req: &str) -> Result<Vec<BookMark>, Error> {
        use crate::schema::bookmarks::dsl::*;

        Ok(bookmarks.filter(label.eq(&label_req)).get_results(&api.conn)?)
    }

    /// Return any bookmarks with the given folder.
    pub fn find_folder(api: &BookMarksApi, folder_req: i32) -> Result<Vec<BookMark>, Error> {
        use crate::schema::bookmarks::dsl::*;

        Ok(bookmarks.filter(folder.eq(&Some(folder_req))).get_results(&api.conn)?)
    }

    /// Return any bookmarks that are starred.
    pub fn find_starred(api: &BookMarksApi, starred_req: bool) -> Result<Vec<BookMark>, Error> {
        use crate::schema::bookmarks::dsl::*;

        Ok(bookmarks.filter(starred.eq(&starred_req)).get_results(&api.conn)?)
    }

    /// Return any bookmarks without a folder.
    pub fn find_unfoldered(api: &BookMarksApi) -> Result<Vec<BookMark>, Error> {
        use crate::schema::bookmarks::dsl::*;

        Ok(bookmarks.filter(folder.is_null()).load(&api.conn)?)
    }

    /// Return a list of all the tags belonging to this bookmark.
    pub fn tags(&self, api: &BookMarksApi) -> Result<Vec<Tag>, Error> {
        use crate::schema::bookmark_tag_map::dsl::*;
        use crate::schema::tags;
        let mut out_tags = vec![];
        for tag in bookmark_tag_map.filter(bookmark_id.eq(self.id)).load::<BookMarkTagMap>(&api.conn)? {
            out_tags.push(tags::table.find(tag.id).first(&api.conn)?);
        }
        Ok(out_tags)
    }

    /// Get the folder this bookmark belongs to.
    pub fn get_folder(&self, api: &BookMarksApi) -> Result<Option<Folder>, Error> {
        use crate::schema::folders::dsl::*;
        if let Some(folder) = self.folder {
            Ok(Some(folders.find(folder).first(&api.conn)?))
        } else {
            Ok(None)
        }
    }

    /// Assign a tag to this bookmark.
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

    /// Remove a tag assignment from this bookmark.
    pub fn remove_tag(&self, api: &BookMarksApi, t_id: i32) -> Result<(), Error> {
        use crate::schema::bookmark_tag_map::dsl::*;
        let filter = bookmark_tag_map.filter(bookmark_id.eq(self.id))
                                     .filter(tag_id.eq(t_id));
        diesel::delete(filter).execute(&api.conn)?;
        Ok(())
    }

    /// Save any changes to the fields of this bookmark.
    ///
    /// **NOTE:** Must be called after modifing either the
    /// `folder`, `label` or `url` paremeters.
    pub fn save(&self, api: &BookMarksApi) -> Result<(), Error> {
        use diesel::SaveChangesDsl;

        self.save_changes::<BookMark>(&api.conn)?;
        Ok(())
    }
}

/// Used to create a new BookMark.
#[derive(Debug, Insertable)]
#[table_name = "bookmarks"]
pub struct NewBookMark {
    /// The url the bookmark should link to.
    pub url: String,
    /// The label used when displaying this bookmark.
    pub label: Option<String>,
    /// The folder this bookmark belongs to.
    pub folder: Option<i32>,
    /// Whether this BookMark should be Starred in the UI.
    pub starred: bool
}

impl NewBookMark {

    pub fn new<S: Into<String>>(url: S) -> NewBookMark {
        NewBookMark {
            url: url.into(),
            label: None,
            folder: None,
            starred: false
        }
    }
}
