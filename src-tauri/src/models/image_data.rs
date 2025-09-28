// src/Image_Data.rs
use image::DynamicImage;
use crate::models::image_size::{ImageSize};


#[derive(Debug)]
pub struct ImageData {
    image:          DynamicImage,
    pub id:         Option<String>,
    pub filename:   Option<String>,
    pub name:       Option<String>,
    pub artist:     Option<String>,
    image_size:     ImageSize,
}

struct Painting {
    id:         String,
    filename:   String,
    name:       String,
    artist:     String,
    width:      u32,
    height:     u32
}

impl ImageData {

    pub fn new(
        image: DynamicImage, 
        image_size: ImageSize,
    ) -> Self {
        ImageData {
            image,
            id:         None,
            filename:   None,
            name:      None,
            artist:     None,
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

    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
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

    pub fn to_paintings(&mut self) -> Vec<Painting> {
        let paintings: Vec<Painting> = Vec::new();
        for size in self.image_size.get_size() {
            todo!()
        }

        paintings

    }

}