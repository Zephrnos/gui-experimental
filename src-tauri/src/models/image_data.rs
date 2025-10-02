use crate::models::image_size::ImageSize;

// The `DynamicImage` field has been removed to reduce memory usage.
// This struct now only holds metadata about a potential crop.
#[derive(Debug, Clone)]
pub struct ImageData {
    pub id:         Option<String>,
    pub filename:   Option<String>,
    pub name:       Option<String>,
    pub artist:     Option<String>,
    pub image_size: ImageSize,
    pub selected:   bool,
}

impl ImageData {
    // The constructor no longer takes an image, only the size metadata.
    pub fn new(image_size: ImageSize) -> Self {
        ImageData {
            id:         None,
            filename:   None,
            name:       None,
            artist:     None,
            image_size,
            selected:   true,
        }
    }

    // `get_image()` has been removed as the image data is no longer stored here.

    pub fn get_sizes(&self) -> &[(u32, u32)] {
        self.image_size.get_size()
    }
}
