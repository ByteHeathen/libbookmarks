use libbookmarks::*;

#[test]
fn edit_bookmark_label() {
    let new_bookmark = NewBookMark {
        url: "example.com".into(),
        label: None,
        folder: None,
        starred: false
    };
    let api = BookMarksApi::new(Some(":memory:".to_string())).expect("Failed to create bookmarks api");
    api.create_bookmark(new_bookmark).expect("Failed to create new Bookmark");

    assert_eq!(1, api.all_bookmarks().expect("Failed to get all bookmarks").len());
    let mut record = api.get_bookmark(1).expect("Could not get inserted bookmark");
    assert_eq!(record.url, "example.com".to_string());
    assert_eq!(record.label, None);
    assert_eq!(record.folder, None);

    record.label = Some("Other Label".to_string());
    record.save(&api).expect("Failed to save record data");

    let record = api.get_bookmark(1).expect("Could not get inserted bookmark");
    assert_eq!(record.url, "example.com".to_string());
    assert_eq!(record.label, Some("Other Label".to_string()));
    assert_eq!(record.folder, None);

    api.remove_bookmark(1).expect("Failed to remove bookmark");
    assert_eq!(0, api.all_bookmarks().expect("Failed to get all bookmarks").len());
}

#[test]
fn edit_bookmark_folder() {
    let new_bookmark = NewBookMark {
        url: "some.url.com".to_string(),
        folder: None,
        label: None,
        starred: false
    };
    let api = BookMarksApi::new(Some(":memory:".to_string())).expect("Failed to create bookmarks api");
    api.create_bookmark(new_bookmark).expect("Failed to create new bookmark");

    assert_eq!(1, api.all_bookmarks().expect("Failed to get all bookmarkss").len());
    let mut record = api.get_bookmark(1).expect("Could not get inserted bookmark");
    assert_eq!(record.label, None);
    assert_eq!(record.folder, None);
    assert_eq!(record.url, "some.url.com".to_string());

    record.folder = Some(1);
    record.save(&api).expect("Failed to save record data");

    let record = api.get_bookmark(1).expect("Could not get inserted bookmark");
    assert_eq!(record.url, "some.url.com".to_string());
    assert_eq!(record.folder, Some(1));
    assert_eq!(record.label, None);

    api.remove_bookmark(1).expect("Failed to remove bookmark");
    assert_eq!(0, api.all_bookmarks().expect("Failed to get all bookmarks").len());
}
