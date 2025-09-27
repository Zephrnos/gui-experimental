use crate::models::{painting::Painting, painting_size::PaintingSize};
use image::{DynamicImage, GenericImageView};

// The main function is now a clean, high-level summary.
pub fn crop_image(image: &mut DynamicImage, size_config: PaintingSize) -> Vec<Painting> {
    // 1. Calculate the optimal crop area based on the target ratio.
    let dimensions_list = size_config.get_size();
    let (width_start, height_start, crop_width, crop_height) =
        calculate_crop_dimensions(image.dimensions(), dimensions_list[0]);

    // 2. Crop the original image in place.
    let cropped_base_image = image.crop_imm(width_start, height_start, crop_width, crop_height);

    // 3. Generate a Painting for each required size from the now-cropped image.
    let mut paintings: Vec<Painting> = Vec::new();
    for (width, height) in dimensions_list {
        let new_painting = Painting::new_from_cropped_image(cropped_base_image.clone(), width, height);
        paintings.push(new_painting);
    }

    paintings
}

// The complex logic is isolated in its own function.
fn calculate_crop_dimensions(image_dims: (u32, u32), target_ratio: (u32, u32)) -> (u32, u32, u32, u32) {
    let (width, height) = image_dims;
    let (img_width, img_height) = target_ratio;

    // Use cross-multiplication to avoid floating point math and integer division errors. Technically more performant?
    // We cast to u64 to prevent overflow when multiplying dimensions.
    let is_image_wider_than_ratio =
        (width as u64 * img_height as u64) >= (height as u64 * img_width as u64);

    let scale_factor: u32 = match is_image_wider_than_ratio {
        // If the image is wider than the target ratio, the height is the limiting dimension.
        true => height / img_height,
        // If the image is taller, the width is the limiting dimension.
        false => width / img_width,
    };

    let crop_width: u32 = img_width * scale_factor;
    let crop_height: u32 = img_height * scale_factor;
    let width_start: u32 = (width - crop_width) / 2;
    let height_start: u32 = (height - crop_height) / 2;

    (width_start, height_start, crop_width, crop_height)
}