use std::sync::Mutex;
use crate::models::painting::Painting;

pub struct AppState {
    pub paintings: Mutex<Vec<Painting>>,
    pub path_to_images: Mutex<Vec<String>>,
}