use libbookmarks::*;

#[test]
fn create_new_no_parent() {
    let new_folder = NewFolder {
        label: "Some Label".into(),
        parent: None
    };
    let api = BookMarksApi::new(Some(":memory:".to_string())).expect("Failed to create bookmarks api");
    api.create_folder(new_folder).expect("Failed to create new folder");

    assert_eq!(1, api.all_folders().expect("Failed to get all folders").len());
    let record = api.get_folder(1).expect("Could not get inserted folder");
    assert_eq!(record.label, "Some Label".to_string());
    assert_eq!(record.parent, None);

    api.remove_folder(1).expect("Failed to remove bookmark");
    assert_eq!(0, api.all_folders().expect("Failed to get all bookmarks").len());
}

#[test]
fn create_new_with_parent() {
    let new_folder1 = NewFolder {
        label: "Some Label".to_string(),
        parent: None
    };
    let new_folder2 = NewFolder {
        label: "Other Label".to_string(),
        parent: Some(1)
    };
    let api = BookMarksApi::new(Some(":memory:".to_string())).expect("Failed to create bookmarks api");
    api.create_folder(new_folder1).expect("Failed to create new tag");
    api.create_folder(new_folder2).expect("Failed to add name");

    assert_eq!(2, api.all_folders().expect("Failed to get all tags").len());
    let record = api.get_folder(2).expect("Could not get inserted tag");
    assert_eq!(record.label, "Other Label".to_string());
    assert_eq!(record.parent, Some(1));

    api.remove_folder(1).expect("Failed to remove bookmark");
    api.remove_folder(2).expect("Failed to remove bookmark");
    assert_eq!(0, api.all_folders().expect("Failed to get all bookmarks").len());
}
