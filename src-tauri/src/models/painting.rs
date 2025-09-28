// src/painting.rs
use image::DynamicImage;
use serde::Serialize;

use crate::models::painting_size::{self, PaintingSize};

#[derive(Serialize, Debug)]
pub struct Painting {
    #[serde(skip)]
    painting: DynamicImage,
    pub id: Option<String>,
    pub filename: Option<String>,
    pub title: Option<String>,
    pub artist: Option<String>,
    painting_size: PaintingSize,
}

impl Painting {

    pub fn new(
        painting: DynamicImage, 
        painting_size: PaintingSize,
    ) -> Self {
        Painting {
            painting,
            id: None,
            filename: None,
            title: None,
            artist: None,
            painting_size,
        }
    }

    pub fn id(mut self, id: String) -> Self {
        self.id = Some(id);
        self
    }

    pub fn filename(mut self, filename: String) -> Self {
        self.filename = Some(filename);
        self
    }

    pub fn title(mut self, title: String) -> Self {
        self.title = Some(title);
        self
    }

    pub fn artist(mut self, artist: String) -> Self {
        self.artist = Some(artist);
        self
    }

    pub fn get_filename(&self) -> &String {
        let to_write = self.filename.as_ref().unwrap();
        to_write
    }

    pub fn get_painting(&self) -> &DynamicImage {
        &self.painting
    }

}