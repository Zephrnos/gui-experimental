use crate::models::painting_list::PaintingList;

pub struct AppState {
    pub pack_metadata: PaintingList<()>,
    pub image_filepaths: Vec<String>,
}