// src/Image_Data.rs
use image::DynamicImage;
use serde::Serialize;

use crate::models::image_size::{ImageSize};

#[derive(Debug)]
pub struct ImageData {
    image: DynamicImage,
    pub id: Option<String>,
    pub filename: Option<String>,
    pub title: Option<String>,
    pub artist: Option<String>,
    image_size: ImageSize,
}

impl Serialize for ImageData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer, 
    {
        
        let mut state = serializer.serialize_struct("Image_Data", 6)?;
        todo!()

    }
}

impl ImageData {

    pub fn new(
        image: DynamicImage, 
        image_size: ImageSize,
    ) -> Self {
        ImageData {
            image,
            id: None,
            filename: None,
            title: None,
            artist: None,
            image_size,
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

    pub fn get_image(&self) -> &DynamicImage {
        &self.image
    }

}