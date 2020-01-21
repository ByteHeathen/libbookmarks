extern crate libbookmarks;

use libbookmarks::Error;
use libbookmarks::NewFolder;
use libbookmarks::BookMarksApi;

fn main() -> Result<(), Error>{
    let api = BookMarksApi::new(Some("test.db".into()))?;
    let new_folder = NewFolder { label: "New Folder".into(), parent: None };
    api.create_folder(new_folder)?;
    println!("{:?}", api.all_folders()?);
    Ok(())
}
