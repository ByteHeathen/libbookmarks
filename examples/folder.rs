extern crate libbookmarks;

use libbookmarks::Error;
use libbookmarks::NewFolder;
use libbookmarks::BookMarksApi;

use std::path::PathBuf;

fn main() -> Result<(), Error>{
    let path = PathBuf::from("test.db");
    if path.exists() {
        std::fs::remove_file(&path)?;
    }
    let api = BookMarksApi::new(Some(&path))?;
    let new_folder = NewFolder { label: "New Folder".into(), parent: None };
    api.create_folder(new_folder)?;
    println!("{:?}", api.all_folders()?);
    Ok(())
}
