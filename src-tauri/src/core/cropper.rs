use crate::models::{painting::{self, Painting}, painting_size::PaintingSize};
use image::{DynamicImage, GenericImageView, imageops::crop};

pub fn crop_image(image: DynamicImage, crop_data: PaintingSize) -> Vec<Painting> {

    let paintings: Vec<Painting> = Vec::new();
    
    let crops = crop_data.get_size();

    let (width, height) = image.dimensions();
    let (ratio_x, ratio_y) = &crops[0];

    let magic_number = match 
      (ratio_x / ratio_y) >= (width / height) {
        true => width / ratio_x,
        false => height / ratio_y,
    };

    let crop_width = ratio_x * magic_number;
    let crop_height = ratio_y * magic_number;
    let width_start = (width - crop_width) / 2;
    let height_start = (height - crop_height) / 2;

    
    for crop_size in crops{
        crop(&mut image.clone(), width_start, height_start, crop_width, crop_height);
    }

    todo!()
}