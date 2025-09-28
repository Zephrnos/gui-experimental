use std::sync::Mutex;
use crate::models::{painting_list::PaintingList};


pub struct AppState {
    pub paintings: Mutex<PaintingList>,
    pub paths: Mutex<Vec<String>>
}