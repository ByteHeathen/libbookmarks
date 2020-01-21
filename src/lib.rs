#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_migrations;

pub mod models;
pub use self::models::*;

pub mod schema;

mod error;
pub use self::error::Error;

mod api;
pub use self::api::BookMarksApi;

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use super::models::*;
    use super::BookMarksApi;

    #[test]
    fn it_works() {
        let api = BookMarksApi::new(Some(&PathBuf::from(":memory:"))).expect("Failed to create api");
        println!("1: {:?}", api.all_tags());
        println!("2: {:?}", api.all_folders());
        println!("3: {:?}", api.all_bookmarks());
        let first_tag = NewTag { label: "New Tag".into(), color: None };
        let first_folder = NewFolder { label: "folder-name".into(), parent: None };
        let first_bookmark = NewBookMark { url: "example.com".into(), label: None, folder: None };
        api.create_tag(first_tag).expect("Failed to add new tag");
        api.create_folder(first_folder).expect("Failed to add new folder");
        api.create_bookmark(first_bookmark).expect("Failed to add new bookmark");
        println!("4: {:?}", api.all_tags());
        println!("5: {:?}", api.all_folders());
        println!("6: {:?}", api.all_bookmarks());
        let bk_folder_one = api.get_folder(1).expect("Failed to get bk folder one");
        let mut bk_one = api.get_bookmark(1).expect("Failed to get bk one");
        bk_one.folder = Some(bk_folder_one.id);
        bk_one.save(&api).expect("Failed to add ");
        println!("{:?}", bk_folder_one.bookmarks(&api));
    }
}
