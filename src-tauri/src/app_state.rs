use crate::models::{image_data::ImageData, painting_list::PaintingList};

enum ListType<T> {
    Images(PaintingList<ImageData>),
    Paintings(PaintingList<T>)
}

pub struct AppState<T> {
    pub input_paths: Vec<String>,
    pub export_path: String,
    pub image_list: ListType<T>
}