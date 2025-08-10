use std::path::Path;

use chrono::{Datelike, NaiveDate, Weekday};
use image::{GenericImageView, imageops};
use log::{info, warn};
use rand::Rng;
use serde::Deserialize;
use tokio::fs;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use tokio::io::{AsyncBufReadExt, BufReader};

use crate::error::{Error, Result};
use crate::git::repo::Repo;
use crate::utils::DateRangeIter;
use crate::visualizer::font::{CHAR_HEIGHT, Char, Font, Pixel};

#[derive(Debug, PartialEq, Eq, Clone, Copy, clap::ValueEnum, Deserialize)]
pub enum CommitCount {
    Zero = 0,
    Few = 1,
    Some = 2,
    Many = 3,
    ALot = 4,
}

impl CommitCount {
    pub fn value(self) -> usize {
        match self {
            CommitCount::Zero => 0,
            CommitCount::Few => 1,
            CommitCount::Some => 2,
            CommitCount::Many => 3,
            CommitCount::ALot => 4,
        }
    }
}

#[derive(Debug)]
pub struct CommitGrid {
    start_date: NaiveDate,
    data: Vec<CommitCount>, // store data by date. data[0] is start_date, data[1] is start_date + 1, etc
}

impl CommitGrid {
    pub async fn populate_repo(&self, repo: &mut Repo) -> Result<()> {
        info!("Populating repo at {:?}", repo.path);
        let date_range = DateRangeIter::new(self.start_date, self.data.len());

        for (i, date) in date_range.enumerate() {
            let commit_count = self.data[i];
            info!("Creating {commit_count:?} commits for date {date:?}");
            for _ in 0..commit_count.value() {
                repo.add_commit(date).await?
            }
        }

        Ok(())
    }

    pub fn new(start_date: NaiveDate) -> Self {
        if start_date.weekday() != Weekday::Sun {
            warn!(
                "{start_date} is not a Sunday. Github starts a week on Sunday, so start_date should be a Sunday as well."
            )
        }
        Self {
            start_date,
            data: vec![],
        }
    }

    pub fn full(&mut self, commit_count: CommitCount, end_date: NaiveDate) -> Result<()> {
        if self.start_date.gt(&end_date) {
            return Err(Error::InvalidArg(
                "end_date must be >= start_date".to_string(),
            ));
        }
        let days = (end_date - self.start_date).num_days() as usize;
        self.data = vec![commit_count; days + 1];
        Ok(())
    }

    pub fn random(&mut self, end_date: NaiveDate) -> Result<()> {
        // randomly pick a CommitCount for each day from start_date to end_date,
        // and populate self.data accordingly
        if self.start_date.gt(&end_date) {
            return Err(Error::InvalidArg(
                "end_date must be >= start_date".to_string(),
            ));
        }

        let days = (end_date - self.start_date).num_days() as usize;
        let mut rng = rand::rng();

        self.data = (0..=days)
            .map(|_| {
                let random_value = rng.random_range(0..=4);
                match random_value {
                    0 => CommitCount::Zero,
                    1 => CommitCount::Few,
                    2 => CommitCount::Some,
                    3 => CommitCount::Many,
                    4 => CommitCount::ALot,
                    _ => unreachable!(),
                }
            })
            .collect();

        Ok(())
    }

    pub async fn read_pattern_file(&mut self, path: impl AsRef<Path>) -> Result<()> {
        // read a file and populate self.data accordingly
        // file format: a grid consist of 0 1 2 3 4 represent CommitCount
        // there must be <= 7 rows (represent 7 days in week).
        // if rows < 7, fill the remaining rows with 0

        let file = fs::File::open(path).await?;
        let reader = BufReader::new(file);
        let mut lines = reader.lines();

        let mut grid: Vec<Vec<CommitCount>> = Vec::new();

        // Read up to 7 rows
        while let Some(line) = lines.next_line().await? {
            if grid.len() >= 7 {
                break;
            }

            let row: Vec<CommitCount> = line
                .chars()
                .map(|num| match num {
                    '0' => CommitCount::Zero,
                    '1' => CommitCount::Few,
                    '2' => CommitCount::Some,
                    '3' => CommitCount::Many,
                    '4' => CommitCount::ALot,
                    _ => CommitCount::Zero, // Default to Zero for invalid values
                })
                .collect();

            if !row.is_empty() {
                grid.push(row);
            }
        }

        let Some(max_len) = grid.iter().map(|x| x.len()).max() else {
            return Err(Error::InvalidData("Empty file".to_string()));
        };

        grid.iter_mut()
            .for_each(|row| row.resize(max_len, CommitCount::Zero));

        // Fill remaining rows with zeros if needed
        while grid.len() < 7 {
            grid.push(vec![CommitCount::Zero; max_len]);
        }

        for c in 0..max_len {
            for row in grid.iter() {
                self.data.push(row[c])
            }
        }

        Ok(())
    }

    pub async fn read_image_file(&mut self, path: impl AsRef<Path>) -> Result<()> {
        // read an image (jpeg, png, etc) -> convert to black and white -> resize to 7 pixel rows (keep the aspect ratio)
        // -> convert each pixel to CommitCount using the pixel brightness

        // Read the image file
        let mut file = File::open(path).await?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).await?;

        // Load the image
        let img = image::load_from_memory(&buffer)
            .map_err(|e| Error::InvalidData(format!("Failed to load image: {e}")))?;

        // Convert to grayscale
        let grayscale = img.grayscale();

        // Resize to 7 rows, keeping aspect ratio
        let (width, height) = grayscale.dimensions();
        let new_height = 7;
        let new_width = (width as f32 * (new_height as f32 / height as f32)).round() as u32;

        let resized = imageops::resize(
            &grayscale,
            new_width,
            new_height,
            imageops::FilterType::Lanczos3,
        );

        // Convert each pixel to CommitCount based on brightness
        let mut commit_counts = Vec::with_capacity(new_width as usize * new_height as usize);
        for x in 0..new_width {
            for y in 0..new_height {
                let pixel = resized.get_pixel(x, y);
                let brightness = pixel[0] as f32 / 255.0; // Normalize to 0.0-1.0

                // Map brightness to CommitCount
                let commit_count = match brightness {
                    b if b < 0.2 => CommitCount::ALot,
                    b if b < 0.4 => CommitCount::Many,
                    b if b < 0.6 => CommitCount::Some,
                    b if b < 0.8 => CommitCount::Few,
                    _ => CommitCount::Zero,
                };

                commit_counts.push(commit_count);
            }
        }

        self.data = commit_counts;

        Ok(())
    }

    pub fn show_text(&mut self, text: String, font: Font, commit_count: CommitCount) -> Result<()> {
        let mut chars: Vec<&Char> = Vec::with_capacity(text.len());
        for byte in text.as_bytes() {
            if let Some(ch) = font.get_char(*byte) {
                chars.push(ch);
            } else {
                return Err(Error::Command(format!(
                    "Text only accepts {}",
                    font.supported_chars()
                )));
            }
        }

        let mut data = Vec::with_capacity(5 * chars.len() * CHAR_HEIGHT); // estimate length

        for ch in chars.iter() {
            // render char
            for pixel in ch.pixels_by_column() {
                data.push(match pixel {
                    Pixel::On => commit_count,
                    Pixel::Off => CommitCount::Zero,
                });
            }
            // space between chars
            for _ in 0..CHAR_HEIGHT {
                data.push(CommitCount::Zero)
            }
        }

        self.data = data;

        Ok(())
    }
}
