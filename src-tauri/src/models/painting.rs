// src/painting.rs
use image::DynamicImage;
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct Painting {
    #[serde(skip_serializing)]
    painting: DynamicImage,
    pub id: Option<String>,
    pub filename: Option<String>,
    pub title: Option<String>,
    pub artist: Option<String>,
    width: u32,
    height: u32,
}

impl Painting {

    pub fn new(
        painting: DynamicImage, 
        width: u32, 
        height: u32
    ) -> Self {
        Painting {
            painting,
            id: None,
            filename: None,
            title: None,
            artist: None,
            width,
            height,
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


    fn get_painting(&self) -> &DynamicImage {
        &self.painting
    }

}