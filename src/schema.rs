table! {
    tags {
        id -> Integer,
        label -> Varchar,
        color -> Nullable<Varchar>,
    }
}

table! {
    folders {
        id -> Integer,
        label -> Varchar,
        parent -> Nullable<Integer>,
    }
}

table! {
    bookmarks {
        id -> Integer,
        url -> Varchar,
        label -> Nullable<Varchar>,
        folder -> Nullable<Integer>,
        starred -> Bool,
    }
}

table! {
    bookmark_tag_map {
        id -> Integer,
        bookmark_id -> Integer,
        tag_id -> Integer,
    }
}
