extern crate libbookmarks;

use libbookmarks::*;

use std::path::PathBuf;

fn main() -> Result<(), Error>{
    let path = PathBuf::from("test.db");
    if path.exists() {
        std::fs::remove_file(&path)?;
    }
    let mut api = BookMarksApi::new(Some(&path))?;

    let new_tag = NewTag { label: "Boring".into(), color: None };
    api.create_tag(new_tag)?;

    let new_bookmark = NewBookMark { url: "example.com".into(), label: None, folder: None };
    api.create_bookmark(new_bookmark)?;
    let bk = api.get_bookmark(1)?;
    bk.assign_tag(&api, 1)?;
    println!("{:?}", api.all_bookmarks()?);
    println!("{:?}", bk.tags(&api)?);
    Ok(())
}
