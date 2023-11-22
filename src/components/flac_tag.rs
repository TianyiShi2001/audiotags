use crate::*;
use id3::Timestamp;
use metaflac;
use std::str::FromStr;

pub use metaflac::Tag as FlacInnerTag;

impl_tag!(FlacTag, FlacInnerTag, TagType::Flac);

impl<'a> From<AnyTag<'a>> for FlacTag {
    fn from(inp: AnyTag<'a>) -> Self {
        let mut t = FlacTag::default();
        if let Some(v) = inp.title() {
            t.set_title(v)
        }
        if let Some(v) = inp.artists_as_string() {
            t.set_artist(&v)
        }
        if let Some(v) = inp.date {
            t.set_date(v)
        }
        if let Some(v) = inp.year {
            t.set_year(v)
        }
        if let Some(v) = inp.album_title() {
            t.set_album_title(v)
        }
        if let Some(v) = inp.album_artists_as_string() {
            t.set_album_artist(&v)
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
    }
}

impl<'a> From<&'a FlacTag> for AnyTag<'a> {
    fn from(inp: &'a FlacTag) -> Self {
        let tag = Self {
            title: inp.title(),
            artists: inp.artists(),
            date: inp.date(),
            year: inp.year(),
            duration: inp.duration(),
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
            ..Self::default()
        };

        tag
    }
}

impl FlacTag {
    pub fn get_first(&self, key: &str) -> Option<&str> {
        if let Some(Some(v)) = self.inner.vorbis_comments().map(|c| c.get(key)) {
            if !v.is_empty() {
                Some(v[0].as_str())
            } else {
                None
            }
        } else {
            None
        }
    }
    pub fn set_first(&mut self, key: &str, val: &str) {
        self.inner.vorbis_comments_mut().set(key, vec![val]);
    }
    pub fn remove(&mut self, k: &str) {
        self.inner.vorbis_comments_mut().comments.remove(k);
    }
}

impl AudioTagEdit for FlacTag {
    fn title(&self) -> Option<&str> {
        self.get_first("TITLE")
    }
    fn set_title(&mut self, title: &str) {
        self.set_first("TITLE", title);
    }
    fn remove_title(&mut self) {
        self.remove("TITLE");
    }

    fn artist(&self) -> Option<&str> {
        self.get_first("ARTIST")
    }
    fn set_artist(&mut self, artist: &str) {
        self.set_first("ARTIST", artist)
    }
    fn remove_artist(&mut self) {
        self.remove("ARTIST");
    }

    fn date(&self) -> Option<Timestamp> {
        if let Some(Ok(timestamp)) = self.get_first("DATE").map(Timestamp::from_str) {
            Some(timestamp)
        } else {
            None
        }
    }
    fn set_date(&mut self, date: Timestamp) {
        self.set_first("DATE", &date.to_string());
    }
    fn remove_date(&mut self) {
        self.remove("DATE");
    }

    fn year(&self) -> Option<i32> {
        if let Some(Ok(y)) = self.get_first("YEAR").map(|s| s.parse::<i32>()) {
            Some(y)
        } else if let Some(Ok(y)) = self
            .get_first("DATE")
            .map(|s| s.chars().take(4).collect::<String>().parse::<i32>())
        {
            Some(y)
        } else {
            None
        }
    }
    fn set_year(&mut self, year: i32) {
        self.set_first("YEAR", &year.to_string());
    }
    fn remove_year(&mut self) {
        self.remove("YEAR");
        self.remove("DATE");
    }

    fn duration(&self) -> Option<f64> {
        self.inner
            .get_streaminfo()
            .map(|s| s.total_samples as f64 / f64::from(s.sample_rate))
    }

    fn album_title(&self) -> Option<&str> {
        self.get_first("ALBUM")
    }
    fn set_album_title(&mut self, title: &str) {
        self.set_first("ALBUM", title)
    }
    fn remove_album_title(&mut self) {
        self.remove("ALBUM");
    }

    fn album_artist(&self) -> Option<&str> {
        self.get_first("ALBUMARTIST")
    }
    fn set_album_artist(&mut self, v: &str) {
        self.set_first("ALBUMARTIST", v)
    }
    fn remove_album_artist(&mut self) {
        self.remove("ALBUMARTIST");
    }

    fn album_cover(&self) -> Option<Picture> {
        self.inner
            .pictures()
            .find(|&pic| matches!(pic.picture_type, metaflac::block::PictureType::CoverFront))
            .and_then(|pic| {
                Some(Picture {
                    data: &pic.data,
                    mime_type: (pic.mime_type.as_str()).try_into().ok()?,
                })
            })
    }
    fn set_album_cover(&mut self, cover: Picture) {
        self.remove_album_cover();
        let mime = String::from(cover.mime_type);
        let picture_type = metaflac::block::PictureType::CoverFront;
        self.inner
            .add_picture(mime, picture_type, (cover.data).to_owned());
    }
    fn remove_album_cover(&mut self) {
        self.inner
            .remove_picture_type(metaflac::block::PictureType::CoverFront)
    }

    fn composer(&self) -> Option<&str> {
        self.get_first("COMPOSER")
    }
    fn set_composer(&mut self, composer: String) {
        self.set_first("COMPOSER", &composer);
    }
    fn remove_composer(&mut self) {
        self.remove("COMPOSER")
    }

    fn track_number(&self) -> Option<u16> {
        if let Some(Ok(n)) = self.get_first("TRACKNUMBER").map(|x| x.parse::<u16>()) {
            Some(n)
        } else {
            None
        }
    }
    fn set_track_number(&mut self, v: u16) {
        self.set_first("TRACKNUMBER", &v.to_string())
    }
    fn remove_track_number(&mut self) {
        self.remove("TRACKNUMBER");
    }

    // ! not standard
    fn total_tracks(&self) -> Option<u16> {
        if let Some(Ok(n)) = self.get_first("TOTALTRACKS").map(|x| x.parse::<u16>()) {
            Some(n)
        } else {
            None
        }
    }
    fn set_total_tracks(&mut self, v: u16) {
        self.set_first("TOTALTRACKS", &v.to_string())
    }
    fn remove_total_tracks(&mut self) {
        self.remove("TOTALTRACKS");
    }

    fn disc_number(&self) -> Option<u16> {
        if let Some(Ok(n)) = self.get_first("DISCNUMBER").map(|x| x.parse::<u16>()) {
            Some(n)
        } else {
            None
        }
    }
    fn set_disc_number(&mut self, v: u16) {
        self.set_first("DISCNUMBER", &v.to_string())
    }
    fn remove_disc_number(&mut self) {
        self.remove("DISCNUMBER");
    }

    // ! not standard
    fn total_discs(&self) -> Option<u16> {
        if let Some(Ok(n)) = self.get_first("TOTALDISCS").map(|x| x.parse::<u16>()) {
            Some(n)
        } else {
            None
        }
    }
    fn set_total_discs(&mut self, v: u16) {
        self.set_first("TOTALDISCS", &v.to_string())
    }
    fn remove_total_discs(&mut self) {
        self.remove("TOTALDISCS");
    }

    fn genre(&self) -> Option<&str> {
        self.get_first("GENRE")
    }
    fn set_genre(&mut self, v: &str) {
        self.set_first("GENRE", v);
    }
    fn remove_genre(&mut self) {
        self.remove("GENRE");
    }

    fn comment(&self) -> Option<&str> {
        self.get_first("COMMENT")
    }
    fn set_comment(&mut self, v: String) {
        self.set_first("COMMENT", &v);
    }
    fn remove_comment(&mut self) {
        self.remove("COMMENT");
    }
}

impl AudioTagWrite for FlacTag {
    fn write_to(&mut self, file: &mut File) -> crate::Result<()> {
        self.inner.write_to(file)?;
        Ok(())
    }
    fn write_to_path(&mut self, path: &str) -> crate::Result<()> {
        self.inner.write_to_path(path)?;
        Ok(())
    }
}
