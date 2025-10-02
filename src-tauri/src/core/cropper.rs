use crate::models::image_size::ImageSize;
use image::{open, GenericImageView, DynamicImage};

/*
Used as an intermediary function to get proper crop dimensions of a given image. No public use.
*/
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

/*
Generates a vector of all 5 cropped image variants from a single source file path.
This is used to create transient images for Base64 preview generation.
These images are NOT stored in the application state to conserve memory.
*/
pub fn generate_cropped_images(path: &str) -> Result<Vec<DynamicImage>, image::ImageError> {
    let mut cropped_images: Vec<DynamicImage> = Vec::new();
    let img = open(path)?;
    let img_dims = img.dimensions();

    for size_variant in ImageSize::iter() {
        let target_size = size_variant.get_size()[0];
        let (width_start, height_start, crop_width, crop_height) =
            calculate_crop_dimensions(img_dims, target_size);

        let crop_view = img.view(width_start, height_start, crop_width, crop_height);
        let crop_preview = DynamicImage::ImageRgba8(crop_view.to_image());

        cropped_images.push(crop_preview);
    }
    Ok(cropped_images)
}

/*
Generates a single cropped image variant from a source file path.
This is used during the final export process to re-generate images on-demand.
*/
pub fn crop_single_image(
    path: &str,
    image_size: &ImageSize,
) -> Result<DynamicImage, image::ImageError> {
    let img = open(path)?;
    let img_dims = img.dimensions();
    let target_size = image_size.get_size()[0];

    let (width_start, height_start, crop_width, crop_height) =
        calculate_crop_dimensions(img_dims, target_size);

    let crop_view = img.view(width_start, height_start, crop_width, crop_height);
    Ok(DynamicImage::ImageRgba8(crop_view.to_image()))
}
