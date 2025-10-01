// src/lib.rs

pub mod models;
pub mod core;
pub mod commands;
pub mod app_state; 

use app_state::AppState;
use std::sync::Mutex;
use tauri::menu::{MenuBuilder, MenuItemBuilder, SubmenuBuilder};
use tauri::{App, Emitter};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    // MANAGE your AppState
    .manage(Mutex::new(AppState::default()))
    // SETUP the menu when the app starts
    .setup(|app| {
        build_menu(app)?;
        Ok(())
    })
    // REGISTER all your commands
    .invoke_handler(tauri::generate_handler![
      commands::my_custom_command,
      commands::open_and_process_images,
      commands::set_selected,
      commands::update_row_metadata,
      commands::update_pack_metadata,
      commands::export_pack
    ])
    // EMIT events from your menu to the frontend
    .on_menu_event(|app_handle, event| {
        match event.id().as_ref() {
            "quit" => { 
              println!("[COMMAND] quit received lib.rs");
              std::process::exit(0); 
            }
            "open_file" => { 
              println!("[COMMAND] open_file received lib.rs");
              app_handle.emit("menu:open_file", ()).unwrap(); 
            }
            "export_pack" => { 
              println!("[COMMAND] export_pack received lib.rs");
              app_handle.emit("menu:export_pack", ()).unwrap(); 
            }
            _ => {}
        }
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

fn build_menu(app: &App) -> tauri::Result<()> {
    let quit_item = MenuItemBuilder::new("Quit").id("quit").build(app)?;
    let open_item = MenuItemBuilder::new("Open Image(s)").id("open_file").build(app)?;
    let export_item = MenuItemBuilder::new("Export Pack").id("export_pack").build(app)?;

    let file_menu = SubmenuBuilder::new(app, "File")
        .item(&open_item)
        .build()?;

    let pack_menu = SubmenuBuilder::new(app, "Pack")
        .item(&export_item)
        .build()?;

    let menu = MenuBuilder::new(app)
      .item(&file_menu)
      .item(&pack_menu)
      .item(&quit_item)
      .build()?;

    app.set_menu(menu)?;

    Ok(())
}