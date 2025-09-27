// src/painting.rs
use crate::painting_size::PaintingSize;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Painting {
    pub id: String,
    pub filename: String,
    pub title: String,
    pub artist: String,
    pub painting_size: PaintingSize,
}

impl Painting {

    fn process_data(input: String) -> Vec<String> {
        todo!()
    }

    pub fn new(input: String, size: PaintingSize) -> Self {
        todo!()
    }

}