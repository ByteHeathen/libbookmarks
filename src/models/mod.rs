//! This module holds all the diesel models
//! used for accessing bookmark data.

mod tag;
pub use self::tag::Tag;
pub use self::tag::NewTag;

mod folder;
pub use self::folder::Folder;
pub use self::folder::NewFolder;

mod bookmark;
pub use self::bookmark::BookMark;
pub use self::bookmark::NewBookMark;
