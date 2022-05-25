use crate::*;

#[derive(Default)]
pub struct AnyTag<'a> {
    pub config: Config,
    pub title: Option<&'a str>,
    pub artists: Option<Vec<&'a str>>,
    pub year: Option<i32>,
    pub duration: Option<f64>,
    pub album_title: Option<&'a str>,
    pub album_artists: Option<Vec<&'a str>>,
    pub album_cover: Option<Picture<'a>>,
    pub track_number: Option<u16>,
    pub total_tracks: Option<u16>,
    pub disc_number: Option<u16>,
    pub total_discs: Option<u16>,
    pub genre:Option<&'a str>,
}

impl AudioTagConfig for AnyTag<'_> {
    fn config(&self) -> &Config {
        &self.config
    }
    fn set_config(&mut self, config: Config) {
        self.config = config.clone();
    }
}

impl<'a> AnyTag<'a> {
    pub fn title(&self) -> Option<&str> {
        self.title.as_deref()
    }
    pub fn set_title(&mut self, title: &'a str) {
        self.title = Some(title);
    }
    pub fn artists(&self) -> Option<&[&str]> {
        self.artists.as_deref()
    }
    // set_artists; add_artist
    pub fn year(&self) -> Option<i32> {
        self.year
    }
    pub fn set_year(&mut self, year: i32) {
        self.year = Some(year);
    }
    pub fn duration(&self) -> Option<f64> {
        self.duration
    }
    pub fn set_duration(&mut self, duration: f64) {
        self.duration = Some(duration);
    }
    pub fn album_title(&self) -> Option<&str> {
        self.album_title.as_deref()
    }
    pub fn album_artists(&self) -> Option<&[&str]> {
        self.album_artists.as_deref()
    }
    pub fn track_number(&self) -> Option<u16> {
        self.track_number
    }
    pub fn total_tracks(&self) -> Option<u16> {
        self.total_tracks
    }
    pub fn disc_number(&self) -> Option<u16> {
        self.track_number
    }
    pub fn total_discs(&self) -> Option<u16> {
        self.total_tracks
    }
    pub fn genre(&self) -> Option<&str> {
        self.genre.as_deref()
    }
}

impl AnyTag<'_> {
    pub fn artists_as_string(&self) -> Option<String> {
        self.artists()
            .map(|artists| artists.join(self.config.sep_artist))
    }
    pub fn album_artists_as_string(&self) -> Option<String> {
        self.album_artists()
            .map(|artists| artists.join(self.config.sep_artist))
    }
}
