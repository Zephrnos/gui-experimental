use std::sync::Mutex;
use crate::models::painting::Painting; // Make sure the path is correct

pub struct AppState {
    pub paintings: Mutex<Vec<Painting>>,
}