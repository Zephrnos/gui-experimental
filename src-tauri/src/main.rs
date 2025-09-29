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
            match event.id().as_ref() {
              "quit" => {std::process::exit(0);}
              "open_file" => {}
              "export_pack" => {}
              "import_pack" => {}
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
    let import_item = MenuItemBuilder::new("Import Pack (Unimplimented)").id("import_pack").build(app)?;

    let file_menu = SubmenuBuilder::new(app, "File")
        .item(&open_item)
        .build()?;

    let pack_menu = SubmenuBuilder::new(app, "Pack")
        .item(&export_item)
        .item(&import_item)
        .build()?;

    let menu = MenuBuilder::new(app)
      .item(&file_menu)
      .item(&pack_menu)
      .item(&quit_item)
      .build()?;

    app.set_menu(menu)?;

    Ok(())
}