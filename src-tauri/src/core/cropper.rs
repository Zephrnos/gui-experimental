use crate::models::{painting::Painting, painting_size::PaintingSize};
use image::{open, DynamicImage, GenericImageView};

// The main function is now a clean, high-level summary.
pub fn crop_preview(path: String) -> Vec<Painting> {

    let mut previews: Vec<Painting> = Vec::new();
    let img = open(path).expect("This was not intended to fail");
    let img_dims = img.dimensions();


    for size_variant in PaintingSize::iter() {
        let target_size = size_variant.get_size()[0];
        let (width_start, height_start, crop_width, crop_height) = 
            calculate_crop_dimensions(img_dims, target_size);
        
        let crop_preview = img.clone().crop_imm(width_start, height_start, crop_width, crop_height);
        
        let new_painting = Painting::new(crop_preview, *size_variant);
        previews.push(new_painting);

    }

    previews

}

// The complex logic is isolated in its own function.
fn calculate_crop_dimensions(image_dims: (u32, u32), target_size: (u32, u32)) -> (u32, u32, u32, u32) {
    let (width, height) = image_dims;
    let (img_width, img_height) = target_size;

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