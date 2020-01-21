use libbookmarks::*;

#[test]
fn create_new_no_label_no_folder() {
    let new_bookmark = NewBookMark {
        url: "example.com".into(),
        label: None,
        folder: None
    };
    let api = BookMarksApi::new(Some(":memory:".to_string())).expect("Failed to create bookmarks api");
    api.create_bookmark(new_bookmark).expect("Failed to create new folder");

    assert_eq!(1, api.all_bookmarks().expect("Failed to get all bookmarks").len());
    let record = api.get_bookmark(1).expect("Could not get inserted bookmark");
    assert_eq!(record.url, "example.com".to_string());
    assert_eq!(record.label, None);
    assert_eq!(record.folder, None);

    api.remove_bookmark(1).expect("Failed to remove bookmark");
    assert_eq!(0, api.all_bookmarks().expect("Failed to get all bookmarks").len());

}

#[test]
fn create_new_label_no_folder() {
    let new_bookmark = NewBookMark {
        url: "example.com".into(),
        label: Some("Some Label".into()),
        folder: None
    };
    let api = BookMarksApi::new(Some(":memory:".to_string())).expect("Failed to create bookmarks api");
    api.create_bookmark(new_bookmark).expect("Failed to create new folder");

    assert_eq!(1, api.all_bookmarks().expect("Failed to get all bookmarks").len());
    let record = api.get_bookmark(1).expect("Could not get inserted bookmark");
    assert_eq!(record.url, "example.com".to_string());
    assert_eq!(record.label, Some("Some Label".to_string()));
    assert_eq!(record.folder, None);

    api.remove_bookmark(1).expect("Failed to remove bookmark");
    assert_eq!(0, api.all_bookmarks().expect("Failed to get all bookmarks").len());
}

#[test]
fn create_new_no_label_folder() {
    let new_bookmark1 = NewBookMark {
        url: "example.com".into(),
        label: Some("Some Label".into()),
        folder: None
    };
    let new_bookmark2 = NewBookMark {
        url: "sub.example.com".into(),
        label: None,
        folder: Some(1)
    };
    let api = BookMarksApi::new(Some(":memory:".to_string())).expect("Failed to create bookmarks api");
    api.create_bookmark(new_bookmark1).expect("Failed to create new bookmark");
    api.create_bookmark(new_bookmark2).expect("Failed to create new bookmark");

    assert_eq!(2, api.all_bookmarks().expect("Failed to get all bookmarks").len());
    let record = api.get_bookmark(2).expect("Could not get inserted bookmark");
    assert_eq!(record.url, "sub.example.com".to_string());
    assert_eq!(record.label, None);
    assert_eq!(record.folder, Some(1));

    api.remove_bookmark(1).expect("Failed to remove bookmark");
    api.remove_bookmark(2).expect("Failed to remove bookmark");
    assert_eq!(0, api.all_bookmarks().expect("Failed to get all bookmarks").len());
}

#[test]
fn create_new_label_folder() {
    let new_bookmark1 = NewBookMark {
        url: "example.com".into(),
        label: Some("Some Label".into()),
        folder: None
    };
    let new_bookmark2 = NewBookMark {
        url: "other.example.com".into(),
        label: Some("Some Label".into()),
        folder: Some(1)
    };
    let api = BookMarksApi::new(Some(":memory:".to_string())).expect("Failed to create bookmarks api");
    api.create_bookmark(new_bookmark1).expect("Failed to create new bookmark");
    api.create_bookmark(new_bookmark2).expect("Failed to create new bookmark");

    assert_eq!(2, api.all_bookmarks().expect("Failed to get all bookmarks").len());
    let record = api.get_bookmark(2).expect("Could not get inserted bookmark");
    assert_eq!(record.url, "other.example.com".to_string());
    assert_eq!(record.label, Some("Some Label".to_string()));
    assert_eq!(record.folder, Some(1));

    api.remove_bookmark(1).expect("Failed to remove bookmark");
    api.remove_bookmark(2).expect("Failed to remove bookmark");
    assert_eq!(0, api.all_bookmarks().expect("Failed to get all bookmarks").len());
}
