use tauri::{AppHandle, Manager}; // Make sure Manager is in scope
use std::fs;

use crate::core::{cropper, exporter};

#[tauri::command]
pub async fn generate_and_save_previews(app: AppHandle, source_path: String) -> Result<Vec<String>, String> {
    // 1. Get the path to the app's local data directory
    let app_data_dir = app.path()
        .app_data_dir()
        .or_else(|_| Err("Failed to get app data directory.".to_string()))?;

    // 2. Create a dedicated, unique folder for these previews to avoid conflicts
    let preview_dir = app_data_dir.join("previews");
    fs::create_dir_all(&preview_dir)
        .map_err(|e| format!("Failed to create preview directory: {}", e))?;

    // 3. Call your existing image cropping logic
    let image_data_vec = cropper::crop_preview(source_path);

    // 4. Call your modified save function
    let saved_file_paths = exporter::save_previews(
        &image_data_vec, 
        &preview_dir.to_string_lossy()
    );

    // 5. Return the list of file paths to the frontend
    Ok(saved_file_paths)
}

#[tauri::command]
async fn open_file() -> Option<Vec<String>> {
    // Use the `rfd` crate to open an async file dialog that allows multiple selections
    let files = rfd::AsyncFileDialog::new()
        .set_title("Choose files...")
        .pick_files() // Changed from .pick_file() to .pick_files()
        .await;

    // The dialog returns an `Option<Vec<FileHandle>>`. We'll convert it to an `Option<Vec<String>>`.
    files.map(|vec_of_handles| {
        vec_of_handles
            .into_iter()
            .map(|handle| handle.path().to_string_lossy().to_string())
            .collect()
    })
}


#[tauri::command]
pub async fn export_pack(export_path: String) {
  
}