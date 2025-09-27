use crate::models::{painting::Painting, painting_size::PaintingSize};
use image::{DynamicImage, GenericImageView};

pub fn crop_image(image: &mut DynamicImage, painting_sizes: PaintingSize) -> Vec<Painting> {
    
    let mut image: DynamicImage = image.clone();
    let mut images: Vec<Painting> = Vec::new();

    let painting_sizes: Vec<(u32, u32)> = painting_sizes.get_size();
    let (width, height) = image.dimensions();
    let (ratio_x, ratio_y) = &painting_sizes[0];

    let magic_number: u32 = match 
      (ratio_x / ratio_y) >= (width / height) {
        true => width / ratio_x,
        false => height / ratio_y,
    };

    let crop_width: u32 = ratio_x * magic_number;
    let crop_height: u32 = ratio_y * magic_number;
    let width_start: u32 = (width - crop_width) / 2;
    let height_start: u32 = (height - crop_height) / 2;

    image = image.crop_imm(width_start, height_start, crop_width, crop_height);

    for painting_size in painting_sizes {
        let new_painting: Painting = Painting::new_from_cropped_image(image.clone(), painting_size.0, painting_size.1);
        images.push(new_painting);
    }

    images

}