use tauri::{AppHandle, Manager};
use std::{fs};

use crate::core::{cropper, exporter};

/*

First command run when window is opened. 

Init:
 - Open a file explorer window
 - Get filepaths of images we want to work with
 - Returns strings of the filepaths
After:
 - Pass filepaths along to a preview generator [something along the likes of generate_previews()]

*/
#[tauri::command]
pub async fn open_file() -> Option<Vec<String>> {
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

/*

Second command run when a window is opened

Init:
 - Takes in a single string [a single image gets all its previews generated in one go]
After:
 - Returns a Vec<String>, where all teh strings are base64 encodings of the image crops

*/
#[tauri::command]
pub fn generate_previews(source_path: String) -> Result<Vec<String>, String> {
    // 1. Call your existing image cropping logic
    let image_data_vec = cropper::crop_preview(&source_path);

    // 2. Call your function to generate Base64 strings in memory
    let saved_base64_strings = exporter::generate_base64_previews(&image_data_vec);

    // 3. Return the list of Base64 strings to the frontend
    Ok(saved_base64_strings)
}

/*

Final command called when window is opened

Init:
 - Takes in an export path
After: 
 - Writes all the images to the export directory with given data, and closes program

*/
#[tauri::command]
pub async fn export_pack(export_path: String) {

}