pub mod models;
pub mod core;
pub mod commands;
pub mod app_state; 

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
      commands::generate_previews,
      commands::open_file,
      commands::export_pack
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}