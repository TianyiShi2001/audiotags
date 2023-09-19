use crate::*;
use id3::{self, Content, Frame, TagLike, Timestamp};

pub use id3::Tag as Id3v2InnerTag;

impl_tag!(Id3v2Tag, Id3v2InnerTag, TagType::Id3v2);

impl<'a> From<&'a Id3v2Tag> for AnyTag<'a> {
    fn from(inp: &'a Id3v2Tag) -> Self {
        Self {
            config: inp.config,

            title: inp.title(),
            artists: inp.artists(),
            year: inp.year(),
            duration: Some(inp.inner.duration().unwrap() as f64),
            album_title: inp.album_title(),
            album_artists: inp.album_artists(),
            album_cover: inp.album_cover(),
            track_number: inp.track_number(),
            total_tracks: inp.total_tracks(),
            disc_number: inp.disc_number(),
            total_discs: inp.total_discs(),
            genre: inp.genre(),
            composer: inp.composer(),
            comment: inp.comment(),
        }
    }
}

impl<'a> From<AnyTag<'a>> for Id3v2Tag {
    fn from(inp: AnyTag<'a>) -> Self {
        Self {
            config: inp.config,
            inner: {
                let mut t = id3::Tag::new();
                if let Some(v) = inp.title() {
                    t.set_title(v)
                }
                if let Some(v) = inp.artists_as_string() {
                    t.set_artist(&v)
                }
                if let Some(v) = inp.year {
                    t.set_year(v)
                }
                if let Some(v) = inp.album_title() {
                    t.set_album(v)
                }
                if let Some(v) = inp.album_artists_as_string() {
                    t.set_album_artist(&v)
                }
                if let Some(v) = inp.track_number() {
                    t.set_track(v as u32)
                }
                if let Some(v) = inp.total_tracks() {
                    t.set_total_tracks(v as u32)
                }
                if let Some(v) = inp.disc_number() {
                    t.set_disc(v as u32)
                }
                if let Some(v) = inp.total_discs() {
                    t.set_total_discs(v as u32)
                }
                if let Some(v) = inp.genre() {
                    t.set_genre(v)
                }
                t
            },
        }
    }
}

impl<'a> std::convert::TryFrom<&'a id3::frame::Picture> for Picture<'a> {
    type Error = crate::Error;
    fn try_from(inp: &'a id3::frame::Picture) -> crate::Result<Self> {
        let id3::frame::Picture {
            mime_type, data, ..
        } = inp;
        let mime_type: MimeType = mime_type.as_str().try_into()?;
        Ok(Self { data, mime_type })
    }
}

impl AudioTagEdit for Id3v2Tag {
    fn title(&self) -> Option<&str> {
        self.inner.title()
    }
    fn set_title(&mut self, title: &str) {
        self.inner.set_title(title)
    }
    fn remove_title(&mut self) {
        self.inner.remove_title();
    }

    fn artist(&self) -> Option<&str> {
        self.inner.artist()
    }
    fn set_artist(&mut self, artist: &str) {
        self.inner.set_artist(artist)
    }
    fn remove_artist(&mut self) {
        self.inner.remove_artist();
    }

    fn year(&self) -> Option<i32> {
        if self.inner.version() == Version::Id3v23 {
            if let ret @ Some(_) = self.inner.year() {
                return ret;
            }
        }

        self.inner.date_recorded().map(|timestamp| timestamp.year)
    }
    fn set_year(&mut self, year: i32) {
        if self.inner.version() == Version::Id3v23 {
            self.inner.set_year(year);
            return;
        }

        if let Some(mut timestamp) = self.inner.date_recorded() {
            timestamp.year = year;
            self.inner.set_date_recorded(timestamp);
            return;
        }

        self.inner.set_date_recorded(Timestamp {
            year,
            month: None,
            day: None,
            hour: None,
            minute: None,
            second: None,
        });
    }
    fn remove_year(&mut self) {
        self.inner.remove_date_recorded();
        self.inner.remove_year();
    }
    fn duration(&self) -> Option<f64> {
        self.inner.duration().map(f64::from)
    }

    fn album_title(&self) -> Option<&str> {
        self.inner.album()
    }
    fn set_album_title(&mut self, v: &str) {
        self.inner.set_album(v)
    }
    fn remove_album_title(&mut self) {
        self.inner.remove_album();
    }

    fn album_artist(&self) -> Option<&str> {
        self.inner.album_artist()
    }
    fn set_album_artist(&mut self, v: &str) {
        self.inner.set_album_artist(v)
    }
    fn remove_album_artist(&mut self) {
        self.inner.remove_album_artist();
    }

    fn album_cover(&self) -> Option<Picture> {
        self.inner
            .pictures()
            .find(|&pic| matches!(pic.picture_type, id3::frame::PictureType::CoverFront))
            .and_then(|pic| {
                Some(Picture {
                    data: &pic.data,
                    mime_type: (pic.mime_type.as_str()).try_into().ok()?,
                })
            })
    }
    fn set_album_cover(&mut self, cover: Picture) {
        self.remove_album_cover();
        self.inner.add_frame(id3::frame::Picture {
            mime_type: String::from(cover.mime_type),
            picture_type: id3::frame::PictureType::CoverFront,
            description: "".to_owned(),
            data: cover.data.to_owned(),
        });
    }
    fn remove_album_cover(&mut self) {
        self.inner
            .remove_picture_by_type(id3::frame::PictureType::CoverFront);
    }

    fn composer(&self) -> Option<&str> {
        if let Some(Content::Text(text)) = self.inner.get("TCOM").map(Frame::content) {
            return Some(text);
        }

        None
    }
    fn set_composer(&mut self, composer: String) {
        self.inner.add_frame(Frame::text("TCOM", composer));
    }
    fn remove_composer(&mut self) {
        self.inner.remove("TCOM");
    }

    fn track_number(&self) -> Option<u16> {
        self.inner.track().map(|x| x as u16)
    }
    fn set_track_number(&mut self, track: u16) {
        self.inner.set_track(track as u32);
    }
    fn remove_track_number(&mut self) {
        self.inner.remove_track();
    }

    fn total_tracks(&self) -> Option<u16> {
        self.inner.total_tracks().map(|x| x as u16)
    }
    fn set_total_tracks(&mut self, total_track: u16) {
        self.inner.set_total_tracks(total_track as u32);
    }
    fn remove_total_tracks(&mut self) {
        self.inner.remove_total_tracks();
    }

    fn disc_number(&self) -> Option<u16> {
        self.inner.disc().map(|x| x as u16)
    }
    fn set_disc_number(&mut self, disc_number: u16) {
        self.inner.set_disc(disc_number as u32)
    }
    fn remove_disc_number(&mut self) {
        self.inner.remove_disc();
    }

    fn total_discs(&self) -> Option<u16> {
        self.inner.total_discs().map(|x| x as u16)
    }
    fn set_total_discs(&mut self, total_discs: u16) {
        self.inner.set_total_discs(total_discs as u32)
    }
    fn remove_total_discs(&mut self) {
        self.inner.remove_total_discs();
    }

    fn genre(&self) -> Option<&str> {
        self.inner.genre()
    }
    fn set_genre(&mut self, v: &str) {
        self.inner.set_genre(v);
    }
    fn remove_genre(&mut self) {
        self.inner.remove_genre();
    }

    fn comment(&self) -> Option<&str> {
        for comment in self.inner.comments() {
            if comment.description.is_empty() {
                return Some(comment.text.as_str());
            }
        }
        None
    }
    fn set_comment(&mut self, comment: String) {
        self.inner.add_frame(id3::frame::Comment {
            lang: "XXX".to_string(),
            description: "".to_string(),
            text: comment,
        });
    }
    fn remove_comment(&mut self) {
        self.inner.remove("COMM");
    }
}

impl AudioTagWrite for Id3v2Tag {
    fn write_to(&mut self, file: &mut File) -> crate::Result<()> {
        self.inner.write_to(file, id3::Version::Id3v24)?;
        Ok(())
    }
    fn write_to_path(&mut self, path: &str) -> crate::Result<()> {
        self.inner.write_to_path(path, id3::Version::Id3v24)?;
        Ok(())
    }
}

// impl<'a> From<AnyTag<'a>> for Id3Tag {
//     fn from(anytag: AnyTag) -> Self {
//         Self {
//             inner: anytag.into(),
//         }
//     }
// }

// impl From<AnyTag> for id3::Tag {
//     fn from(anytag: AnyTag) -> Self {
//         let mut id3tag = Self::default();
//         anytag
//             .artists_as_string(SEP_ARTIST)
//             .map(|v| id3tag.set_artist(v));
//         anytag.year().map(|v| id3tag.set_year(v));
//         anytag.album_title().map(|v| id3tag.set_album(v));
//         anytag
//             .album_artists_as_string(SEP_ARTIST)
//             .map(|v| id3tag.set_album_artist(v));
//         anytag.track_number().map(|v| id3tag.set_track(v as u32));
//         anytag
//             .total_tracks()
//             .map(|v| id3tag.set_total_tracks(v as u32));
//         anytag.disc_number().map(|v| id3tag.set_disc(v as u32));
//         anytag
//             .total_discs()
//             .map(|v| id3tag.set_total_discs(v as u32));
//         id3tag
//     }
// }
