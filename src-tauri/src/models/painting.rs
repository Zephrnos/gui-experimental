// src/painting.rs
use image::DynamicImage;
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct Painting {
    #[serde(skip_serializing)]
    pub painting: DynamicImage,
    pub id: String,
    pub filename: String,
    pub title: String,
    pub artist: String,
    pub width: u32,
    pub height: u32,
}

impl Painting {

    fn set_id(&mut self, id: String) {
        self.id = id;
    }

    fn set_filename(&mut self, filename: String) {
        self.filename = filename;
    }

    pub fn set_title(&mut self, title: String) {
        self.title = title;
    }

    pub fn set_artist(&mut self, artist: String) {
        self.artist = artist;
    }

    pub fn new_from_cropped_image(
        painting: DynamicImage,
        width: u32, 
        height: u32        
    ) -> Self {
        Painting {
            painting,
            id: String::new(),
            filename: String::new(),
            title: String::new(),
            artist: String::new(),
            width,
            height
        }
    }

    pub fn new(
        painting: DynamicImage,
        id: String, 
        filename: String, 
        title: String, 
        artist: String, 
        width: u32, 
        height: u32
    ) -> Self {
        Painting {
            painting,
            id,
            filename,
            title,
            artist,
            width,
            height,
        }
    }

}