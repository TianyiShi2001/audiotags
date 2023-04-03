use crate::*;
use mp4ameta::{self, ImgFmt};

pub use mp4ameta::Tag as Mp4InnerTag;

impl_tag!(Mp4Tag, Mp4InnerTag, TagType::Mp4);

impl<'a> From<&'a Mp4Tag> for AnyTag<'a> {
    fn from(inp: &'a Mp4Tag) -> Self {
        let title = inp.title();
        let artists = inp.artists().map(|i| i.into_iter().collect::<Vec<_>>());
        let year = inp.year();
        let duration = inp.duration();
        let album_title = inp.album_title();
        let album_artists = inp
            .album_artists()
            .map(|i| i.into_iter().collect::<Vec<_>>());
        let album_cover = inp.album_cover();
        let (a, b) = inp.track();
        let track_number = a;
        let total_tracks = b;
        let (a, b) = inp.disc();
        let disc_number = a;
        let total_discs = b;
        let genre = inp.genre();
        let composer = inp.composer();
        let comment = inp.comment();
        Self {
            config: inp.config,
            title,
            artists,
            year,
            duration,
            album_title,
            album_cover,
            album_artists,
            track_number,
            total_tracks,
            disc_number,
            total_discs,
            genre,
            composer,
            comment,
        }
    }
}

impl<'a> From<AnyTag<'a>> for Mp4Tag {
    fn from(inp: AnyTag<'a>) -> Self {
        Self {
            config: inp.config,
            inner: {
                let mut t = mp4ameta::Tag::default();
                if let Some(v) = inp.title() {
                    t.set_title(v)
                }
                if let Some(i) = inp.artists() {
                    i.iter().for_each(|&a| t.add_artist(a))
                }
                if let Some(v) = inp.year {
                    t.set_year(v.to_string())
                }
                if let Some(v) = inp.album_title() {
                    t.set_album(v)
                }
                if let Some(i) = inp.album_artists() {
                    i.iter().for_each(|&a| t.add_album_artist(a))
                }
                if let Some(v) = inp.track_number() {
                    t.set_track_number(v)
                }
                if let Some(v) = inp.total_tracks() {
                    t.set_total_tracks(v)
                }
                if let Some(v) = inp.disc_number() {
                    t.set_disc_number(v)
                }
                if let Some(v) = inp.total_discs() {
                    t.set_total_discs(v)
                }
                t
            },
        }
    }
}

impl<'a> std::convert::TryFrom<&'a mp4ameta::Data> for Picture<'a> {
    type Error = crate::Error;
    fn try_from(inp: &'a mp4ameta::Data) -> crate::Result<Self> {
        Ok(match *inp {
            mp4ameta::Data::Png(ref data) => Self {
                data,
                mime_type: MimeType::Png,
            },
            mp4ameta::Data::Jpeg(ref data) => Self {
                data,
                mime_type: MimeType::Jpeg,
            },
            _ => return Err(crate::Error::NotAPicture),
        })
    }
}

impl AudioTagEdit for Mp4Tag {
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
        self.inner.remove_artists();
    }

    fn artists(&self) -> Option<Vec<&str>> {
        let v = self.inner.artists().fold(Vec::new(), |mut v, a| {
            v.push(a);
            v
        });
        if !v.is_empty() {
            Some(v)
        } else {
            None
        }
    }
    fn add_artist(&mut self, v: &str) {
        self.inner.add_artist(v);
    }

    fn year(&self) -> Option<i32> {
        self.inner.year().and_then(|x| str::parse(x).ok())
    }
    fn set_year(&mut self, year: i32) {
        self.inner.set_year(year.to_string())
    }
    fn remove_year(&mut self) {
        self.inner.remove_year();
    }

    // Return Option with duration in second
    fn duration(&self) -> Option<f64> {
        self.inner.duration().map(|d| d.as_secs_f64())
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
        self.inner.remove_album_artists();
    }

    fn album_artists(&self) -> Option<Vec<&str>> {
        let v = self.inner.album_artists().fold(Vec::new(), |mut v, a| {
            v.push(a);
            v
        });
        if !v.is_empty() {
            Some(v)
        } else {
            None
        }
    }
    fn add_album_artist(&mut self, v: &str) {
        self.inner.add_album_artist(v);
    }

    fn album_cover(&self) -> Option<Picture> {
        self.inner.artwork().and_then(|data| match data.fmt {
            ImgFmt::Jpeg => Some(Picture {
                data: data.data,
                mime_type: MimeType::Jpeg,
            }),
            ImgFmt::Png => Some(Picture {
                data: data.data,
                mime_type: MimeType::Png,
            }),
            _ => None,
        })
    }
    fn set_album_cover(&mut self, cover: Picture) {
        self.remove_album_cover();
        self.inner.add_artwork(match cover.mime_type {
            MimeType::Png => mp4ameta::Img {
                fmt: ImgFmt::Png,
                data: cover.data.to_owned(),
            },
            MimeType::Jpeg => mp4ameta::Img {
                fmt: ImgFmt::Jpeg,
                data: cover.data.to_owned(),
            },
            _ => panic!("Only png and jpeg are supported in m4a"),
        });
    }
    fn remove_album_cover(&mut self) {
        self.inner.remove_artworks();
    }

    fn remove_track(&mut self) {
        self.inner.remove_track(); // faster than removing separately
    }

    fn composer(&self) -> Option<&str> {
        self.inner.composer()
    }
    fn set_composer(&mut self, composer: String) {
        self.inner.set_composer(composer);
    }
    fn remove_composer(&mut self) {
        self.inner.remove_composers();
    }

    fn track_number(&self) -> Option<u16> {
        self.inner.track_number()
    }
    fn set_track_number(&mut self, track: u16) {
        self.inner.set_track_number(track);
    }
    fn remove_track_number(&mut self) {
        self.inner.remove_track_number();
    }

    fn total_tracks(&self) -> Option<u16> {
        self.inner.total_tracks()
    }
    fn set_total_tracks(&mut self, total_track: u16) {
        self.inner.set_total_tracks(total_track);
    }
    fn remove_total_tracks(&mut self) {
        self.inner.remove_total_tracks();
    }

    fn remove_disc(&mut self) {
        self.inner.remove_disc();
    }

    fn disc_number(&self) -> Option<u16> {
        self.inner.disc_number()
    }
    fn set_disc_number(&mut self, disc_number: u16) {
        self.inner.set_disc_number(disc_number)
    }
    fn remove_disc_number(&mut self) {
        self.inner.remove_disc_number();
    }

    fn total_discs(&self) -> Option<u16> {
        self.inner.total_discs()
    }
    fn set_total_discs(&mut self, total_discs: u16) {
        self.inner.set_total_discs(total_discs)
    }
    fn remove_total_discs(&mut self) {
        self.inner.remove_total_discs();
    }

    fn genre(&self) -> Option<&str> {
        self.inner.genre()
    }
    fn set_genre(&mut self, genre: &str) {
        self.inner.set_genre(genre);
    }
    fn remove_genre(&mut self) {
        self.inner.remove_genres();
    }

    fn comment(&self) -> Option<&str> {
        self.inner.comment()
    }
    fn set_comment(&mut self, comment: String) {
        self.inner.set_comment(comment);
    }
    fn remove_comment(&mut self) {
        self.inner.remove_comments();
    }
}

impl AudioTagWrite for Mp4Tag {
    fn write_to(&mut self, file: &mut File) -> crate::Result<()> {
        self.inner.write_to(file)?;
        Ok(())
    }
    fn write_to_path(&mut self, path: &str) -> crate::Result<()> {
        self.inner.write_to_path(path)?;
        Ok(())
    }
}
