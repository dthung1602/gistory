use serde::Deserialize;

use super::font_subway_tracker::CHARS_MAPPING as SUBWAY_TRACKER_MAPPING;

pub const CHAR_HEIGHT: usize = 7;

pub enum Pixel {
    On,
    Off,
}

pub struct Char {
    pub(crate) data: [&'static str; CHAR_HEIGHT],
}

impl Char {
    #[inline]
    pub fn width(&self) -> usize {
        self.data[0].len()
    }

    pub fn pixels_by_column(&self) -> Vec<Pixel> {
        let width = self.width();
        let mut res = Vec::with_capacity(width * CHAR_HEIGHT);

        for i in 0..width {
            for j in 0..CHAR_HEIGHT {
                let pixel = match self.data[j].as_bytes()[i] {
                    b'x' => Pixel::On,
                    b'.' => Pixel::Off,
                    _ => panic!("Invalid char in font"),
                };
                res.push(pixel);
            }
        }
        res
    }
}

#[derive(Debug, Clone, Copy, clap::ValueEnum, Deserialize)]
pub enum Font {
    SubwayTracker,
}

impl Font {
    pub fn get_char(&self, ch: u8) -> Option<&Char> {
        match self {
            Font::SubwayTracker => SUBWAY_TRACKER_MAPPING.get(&ch),
        }
    }

    pub fn supported_chars(&self) -> String {
        let map = match self {
            Font::SubwayTracker => SUBWAY_TRACKER_MAPPING,
        };
        let chars: Vec<u8> = map.keys().copied().collect();
        format!("<space> {}", String::from_utf8(chars).unwrap())
    }
}
