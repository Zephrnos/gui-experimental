// use std::fs::{create_dir_all, write};

// use image::DynamicImage;

// use crate::models::painting_list::PaintingList;

// // Load in the default icon to bianary so the file is contained in the executable
// const DEFAULT_ICON: &[u8] = include_bytes!("../../assets/icon.png");

// fn write_icon(export_path: &String) {
//     create_dir_all(format!("{}/images", export_path)).expect("Failed to create images directory");
//     write(format!("{}/icon.png", export_path), DEFAULT_ICON).expect("Failed to write default icon");
// }

// fn write_json(paintings_list: &PaintingList, export_path: &String) {
//     let json_data = serde_json::to_string_pretty(paintings_list).expect("Failed to serialize painting list");
//     write(format!("{}/custompaintings.json", export_path), json_data).expect("Failed to write painting list JSON file");
// }

// fn write_images(paintings_list: &PaintingList, export_path: &String) {
//     let paintings = paintings_list.retrieve_paintings();
//     for painting in paintings {
//         let img: &DynamicImage = painting.get_image();
//         let _ = img.save(format!("{}/images/{}.png", export_path, painting.get_filename()));
//     }
// }

// pub fn export(paintings_list: PaintingList, export_path: String) {

//     write_icon(&export_path);
//     write_json(&paintings_list, &export_path);
//     write_images(&paintings_list, &export_path);

// }

