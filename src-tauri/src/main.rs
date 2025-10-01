#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app_state;
mod commands;
mod core;
mod models;

use app_state::AppState;
use std::sync::Mutex;
use tauri::menu::{MenuBuilder, MenuItemBuilder, SubmenuBuilder};
use tauri::{App, Emitter}; // Add Manager

fn main() {
    tauri::Builder::default()
        // --- 2. MANAGE your AppState ---
        .manage(Mutex::new(AppState::default()))
        // --- 3. REGISTER your commands ---
        .invoke_handler(tauri::generate_handler![
            commands::open_and_process_images,
            commands::set_selected,
            commands::update_row_metadata,
            commands::update_pack_metadata,
            commands::export_pack
        ])
        .setup(|app| {
            build_menu(app)?;
            Ok(())
        })
        // --- 4. EMIT events from your menu ---
        .on_menu_event(|app_handle, event| {
            match event.id().as_ref() {
                "quit" => { std::process::exit(0); }
                "open_file" => { app_handle.emit("menu:open_file", ()).unwrap(); }
                "export_pack" => { app_handle.emit("menu:export_pack", ()).unwrap(); }
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
    // let import_item = MenuItemBuilder::new("Import Pack (Unimplimented)").id("import_pack").build(app)?;

    let file_menu = SubmenuBuilder::new(app, "File")
        .item(&open_item)
        .build()?;

    let pack_menu = SubmenuBuilder::new(app, "Pack")
        .item(&export_item)
        // .item(&import_item)
        .build()?;

    let menu = MenuBuilder::new(app)
      .item(&file_menu)
      .item(&pack_menu)
      .item(&quit_item)
      .build()?;

    app.set_menu(menu)?;

    Ok(())
}