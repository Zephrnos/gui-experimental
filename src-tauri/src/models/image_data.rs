use image::DynamicImage;
use crate::models::image_size::ImageSize;

#[derive(Debug)]
pub struct ImageData {
    image:          DynamicImage,
    pub id:         Option<String>,
    pub filename:   Option<String>,
    pub name:       Option<String>,
    pub artist:     Option<String>,
    pub image_size:     ImageSize,
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

    pub fn get_sizes(&self) -> Vec<(u32, u32)> {
        self.image_size.get_size()
    }

}