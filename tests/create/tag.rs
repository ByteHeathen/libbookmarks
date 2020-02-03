use libbookmarks::*;

#[test]
fn create_new_no_color() {
    let new_tag = NewTag {
        label: "Some Label".into(),
        color: None
    };
    let api = BookMarksApi::new(Some(":memory:".to_string())).expect("Failed to create bookmarks api");
    api.create_tag(new_tag).expect("Failed to create new tag");

    assert_eq!(1, api.all_tags().expect("Failed to get all tags").len());
    let record = api.get_tag(1).expect("Could not get inserted tag");
    assert_eq!(record.label, "Some Label".to_string());
    assert_eq!(record.color, None);

    api.remove_tag(1).expect("Failed to remove tag");
    assert_eq!(0, api.all_tags().expect("Failed to get all tags").len());
}

#[test]
fn create_new_with_color() {
    let new_tag = NewTag {
        label: "Some Label".to_string(),
        color: Some("#efefef".to_string())
    };
    let api = BookMarksApi::new(Some(":memory:".to_string())).expect("Failed to create bookmarks api");
    api.create_tag(new_tag).expect("Failed to create new tag");

    assert_eq!(1, api.all_tags().expect("Failed to get all tags").len());
    let record = api.get_tag(1).expect("Could not get inserted tag");
    assert_eq!(record.label, "Some Label".to_string());
    assert_eq!(record.color, Some("#efefef".to_string()));

    api.remove_tag(1).expect("Failed to remove tag");
    assert_eq!(0, api.all_tags().expect("Failed to get all tags").len());
}
