// src-tauri/src/main.rs

// Prevents a console window from appearing on Windows in release builds.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::menu::{MenuBuilder, MenuItemBuilder, SubmenuBuilder};
use tauri::App;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            build_menu(app)?;
            Ok(())
        })
        .on_menu_event(|_app_handle, event| { // <-- Corrected to accept two arguments
            if event.id() == "quit" {
                std::process::exit(0);
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn build_menu(app: &App) -> tauri::Result<()> {
    let open_item = MenuItemBuilder::new("Open File").id("OpenFile").build(app)?;
    let build_item = MenuItemBuilder::new("Build Pack").id("BuildPack").build(app)?;

    let file_menu = SubmenuBuilder::new(app, "File")
        .item(&open_item)
        .item(&build_item)
        .build()?;

    let menu = MenuBuilder::new(app).item(&file_menu).build()?;

    app.set_menu(menu)?;

    Ok(())
}