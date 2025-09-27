use std::fs::{create_dir_all, write, self};
use serde::{Deserialize, Serialize};

use crate::models::painting_list::PaintingList;
use crate::models::painting::Painting;

// Load in the default icon to bianary so the file is contained in the executable
const DEFAULT_ICON: &[u8] = include_bytes!("../../assets/icon.png");

pub fn export(paintings: Vec<Painting>, export_path: String) {

    let painting_list = PaintingList::new(paintings);

    let json_data = serde_json::to_string_pretty(&painting_list).expect("Failed to serialize painting list");

    create_dir_all(format!("{}/images", &export_path)).expect("Failed to create images directory");

    write(format!("{}/icon.png", &export_path), DEFAULT_ICON).expect("Failed to write default icon");
    write(format!("{}/custompaintings.json", &export_path), json_data).expect("Failed to write painting list JSON file");

}