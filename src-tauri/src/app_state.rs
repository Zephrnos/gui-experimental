use crate::models::{image_data::ImageData, pack_list::PackList};

/// Represents one row in your UI. It contains the data
/// for one source image and all its generated crops.
#[derive(Debug, Clone)]
pub struct SourceImageGroup {
    pub source_path: String,
    pub name: String,
    pub artist: String,
    pub crops: Vec<ImageData>,
}

/// The single, central state for the entire application.
#[derive(Debug, Default)]
pub struct AppState {
    // This stores global metadata like version and description.
    pub pack_metadata: PackList<()>,

    // This vector holds all the source images and their groups of crops.
    // Each element corresponds to one row in the UI.
    pub image_groups: Vec<SourceImageGroup>,
}