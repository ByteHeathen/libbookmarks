use libbookmarks::*;

#[test]
fn edit_folder_label() {
    let new_folder = NewFolder {
        label: "Some Label".into(),
        parent: None
    };
    let api = BookMarksApi::new(Some(":memory:".to_string())).expect("Failed to create bookmarks api");
    api.create_folder(new_folder).expect("Failed to create new folder");

    assert_eq!(1, api.all_folders().expect("Failed to get all folders").len());
    let mut record = api.get_folder(1).expect("Could not get inserted folder");
    assert_eq!(record.label, "Some Label".to_string());
    assert_eq!(record.parent, None);

    record.label = "Other Label".to_string();
    record.save(&api).expect("Failed to save record data");

    let record = api.get_folder(1).expect("Could not get inserted folder");
    assert_eq!(record.label, "Other Label".to_string());
    assert_eq!(record.parent, None);

    api.remove_folder(1).expect("Failed to remove bookmark");
    assert_eq!(0, api.all_folders().expect("Failed to get all bookmarks").len());
}

#[test]
fn edit_folder_parent() {
    let new_folder = NewFolder {
        label: "Some Label".to_string(),
        parent: None
    };
    let api = BookMarksApi::new(Some(":memory:".to_string())).expect("Failed to create bookmarks api");
    api.create_folder(new_folder).expect("Failed to create new folder");

    assert_eq!(1, api.all_folders().expect("Failed to get all folders").len());
    let mut record = api.get_folder(1).expect("Could not get inserted folder");
    assert_eq!(record.label, "Some Label".to_string());
    assert_eq!(record.parent, None);

    record.parent = Some(1);
    record.save(&api).expect("Failed to save record data");

    let record = api.get_folder(1).expect("Could not get inserted folder");
    assert_eq!(record.label, "Some Label".to_string());
    assert_eq!(record.parent, Some(1));
    api.remove_folder(1).expect("Failed to remove bookmark");
    assert_eq!(0, api.all_folders().expect("Failed to get all bookmarks").len());
}
