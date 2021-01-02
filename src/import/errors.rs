use crate::import::artwork;
use std::io;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Diesel(#[from] diesel::result::Error),

    #[error(transparent)]
    Io(#[from] io::Error),

    #[error(transparent)]
    Artwork(#[from] artwork::Error),

    #[error("either the tag's album artist or artist needs to be set, neither is")]
    NoArtistError,

    #[error("the album name wasn't specified in the tag")]
    NoAlbumError,

    #[error("the title wasn't specified in the tag")]
    NoTitleError,
}
