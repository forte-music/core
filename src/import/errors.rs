error_chain! {
    foreign_links {
        Diesel(::diesel::result::Error);
        Io(::std::io::Error);
    }

    errors {
        NoArtistError {
            description("either the tag's album artist or artist needs to be set, neither is")
        }

        NoAlbumError {
            description("the album name wasn't specified in the tag")
        }

        NoTitleError {
            description("the title wasn't specified in the tag")
        }
    }
}
