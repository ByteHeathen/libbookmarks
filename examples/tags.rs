extern crate libbookmarks;

use libbookmarks::*;

fn main() -> Result<(), Error>{
    let api = BookMarksApi::new(Some("test.db".into()))?;

    let new_tag = NewTag { label: "Boring".into(), color: None };
    api.create_tag(new_tag)?;

    let new_bookmark = NewBookMark::new("example.com");
    api.create_bookmark(new_bookmark)?;
    let bk = api.get_bookmark(1)?;
    bk.assign_tag(&api, 1)?;
    println!("{:?}", api.all_bookmarks()?);
    println!("{:?}", bk.tags(&api)?);
    Ok(())
}
