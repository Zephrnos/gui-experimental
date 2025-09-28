use image::DynamicImage;
use serde::Serialize;
use crate::models::image_size::ImageSize;


#[derive(Debug)]
pub struct ImageData {
    image:          DynamicImage,
    pub id:         Option<String>,
    pub filename:   Option<String>,
    pub name:       Option<String>,
    pub artist:     Option<String>,
    image_size:     ImageSize,
}

#[derive(Serialize)]
pub struct Painting {
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

    pub fn get_image(&self) -> &DynamicImage {
        &self.image
    }

    pub fn to_paintings(&mut self) -> Vec<Painting> {

        let mut paintings: Vec<Painting> = Vec::new();

        for (width, height) in self.image_size.get_size() {

            let id: String = format!("{}_{}x{}", self.id.clone().unwrap(), &width, &height);
            let filename: String = format!("{}_{}x{}", self.filename.clone().unwrap(), &width, &height);

            let painting: Painting = Painting {
                id,
                filename,
                name: self.name.clone().unwrap(),
                artist: self.artist.clone().unwrap(), 
                width, 
                height, 
            };

            paintings.push(painting);
            
        };

        paintings

    }

}