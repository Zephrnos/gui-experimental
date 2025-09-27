// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use app_lib::painting_list::PaintingList;
use app_lib::painting::Painting;
use app_lib::cropper::crop_image;


fn serialize_to_json(image_data: PaintingList) {
  todo!()
}

fn main() {
  app_lib::run();

  // Create a vec of paintings to add paintins to
  let mut paintings: Vec<Painting>  = Vec::new();


}
