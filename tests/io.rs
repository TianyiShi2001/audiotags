use audiotags::{MimeType, Picture, Tag};
use std::ffi::OsString;
use std::fs;
use std::path::Path;
use tempfile::Builder;

macro_rules! test_file {
    ( $function:ident, $file:expr ) => {
        #[test]
        fn $function() {
            let path = Path::new($file);
            let mut suffix = OsString::from(".");
            suffix.push(path.extension().unwrap());
            let tmp = Builder::new().suffix(&suffix).tempfile().unwrap();
            fs::copy($file, &tmp).unwrap();
            let tmp_path = tmp.path();

            let mut tags = Tag::default().read_from_path(tmp_path).unwrap();
            tags.set_title("foo title");
            assert_eq!(tags.title(), Some("foo title"));
            tags.remove_title();
            assert!(tags.title().is_none());
            tags.remove_title(); // should not panic

            tags.set_artist("foo artist");
            assert_eq!(tags.artist(), Some("foo artist"));
            tags.remove_artist();
            assert!(tags.artist().is_none());
            tags.remove_artist();

            tags.set_year(2020);
            assert_eq!(tags.year(), Some(2020));
            tags.remove_year();
            assert!(tags.year().is_none());
            tags.remove_year();

            tags.set_album_title("foo album title");
            assert_eq!(tags.album_title(), Some("foo album title"));
            tags.remove_album_title();
            assert!(tags.album_title().is_none());
            tags.remove_album_title();

            tags.set_album_artist("foo album artist");
            assert_eq!(tags.album_artist(), Some("foo album artist"));
            tags.remove_album_artist();
            assert!(tags.album_artist().is_none());
            tags.remove_album_artist();

            let cover = Picture {
                mime_type: MimeType::Jpeg,
                data: &vec![0u8; 10],
            };

            tags.set_album_cover(cover.clone());
            assert_eq!(tags.album_cover(), Some(cover));
            tags.remove_album_cover();
            assert!(tags.album_cover().is_none());
            tags.remove_album_cover();

            tags.set_genre("foo song genre");
            assert_eq!(tags.genre(), Some("foo song genre"));
            tags.remove_genre();
            assert!(tags.genre().is_none());
            tags.remove_genre();
        }
    };
}

test_file!(test_mp3, "assets/a.mp3");
test_file!(test_m4a, "assets/a.m4a");
test_file!(test_flac, "assets/a.flac");
