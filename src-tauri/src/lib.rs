pub mod models;
pub mod core;
pub mod commands;
pub mod app_state; 

use crate::{app_state::AppState, models::painting_list::PaintingList};
use std::sync::Mutex;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .manage(AppState {
      paintings: Mutex::new(PaintingList::default())
    })
    .invoke_handler(tauri::generate_handler![commands::my_custom_command])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}