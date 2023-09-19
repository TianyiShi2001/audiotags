use crate::*;
use id3::Timestamp;

#[derive(Default)]
pub struct AnyTag<'a> {
    pub config: Config,
    pub title: Option<&'a str>,
    pub artists: Option<Vec<&'a str>>,
    pub date_released: Option<Timestamp>,
    pub original_date_released: Option<Timestamp>,
    pub date_recorded: Option<Timestamp>,
    pub year: Option<i32>,
    pub duration: Option<f64>,
    pub album_title: Option<&'a str>,
    pub album_artists: Option<Vec<&'a str>>,
    pub album_cover: Option<Picture<'a>>,
    pub track_number: Option<u16>,
    pub total_tracks: Option<u16>,
    pub disc_number: Option<u16>,
    pub total_discs: Option<u16>,
    pub genre: Option<&'a str>,
    pub composer: Option<&'a str>,
    pub comment: Option<&'a str>,
}

impl AudioTagConfig for AnyTag<'_> {
    fn config(&self) -> &Config {
        &self.config
    }
    fn set_config(&mut self, config: Config) {
        self.config = config;
    }
}

impl<'a> AnyTag<'a> {
    pub fn title(&self) -> Option<&str> {
        self.title
    }
    pub fn set_title(&mut self, title: &'a str) {
        self.title = Some(title);
    }
    pub fn artists(&self) -> Option<&[&str]> {
        self.artists.as_deref()
    }
    // set_artists; add_artist
    pub fn date_released(&self) -> Option<Timestamp> {
        self.date_released
    }
    pub fn set_date_released(&mut self, date_released: Timestamp) {
        self.date_released = Some(date_released);
    }
    pub fn original_date_released(&self) -> Option<Timestamp> {
        self.original_date_released
    }
    pub fn set_original_date_released(&mut self, original_date_released: Timestamp) {
        self.original_date_released = Some(original_date_released);
    }
    pub fn date_recorded(&self) -> Option<Timestamp> {
        self.date_recorded
    }
    pub fn set_date_recorded(&mut self, date_recorded: Timestamp) {
        self.date_recorded = Some(date_recorded);
    }
    pub fn year(&self) -> Option<i32> {
        self.year
    }
    pub fn set_year(&mut self, year: i32) {
        self.year = Some(year);
    }
    pub fn duration(&self) -> Option<f64> {
        self.duration
    }
    pub fn album_title(&self) -> Option<&str> {
        self.album_title
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
        self.disc_number
    }
    pub fn total_discs(&self) -> Option<u16> {
        self.total_discs
    }
    pub fn genre(&self) -> Option<&str> {
        self.genre
    }
    pub fn composer(&self) -> Option<&str> {
        self.composer
    }
    pub fn comment(&self) -> Option<&str> {
        self.comment
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
