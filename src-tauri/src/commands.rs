use std::sync::Mutex;
use tauri::State;
use crate::{
    app_state::{AppState, SourceImageGroup}, 
    core::{cropper, exporter}
};

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
pub async fn open_and_process_images(state: State<'_, Mutex<AppState>>) -> Result<Vec<String>, String> {
    let files = rfd::AsyncFileDialog::new()
        .set_title("Choose Images...")
        .add_filter("Image Files", &["png", "jpg", "jpeg"])
        .pick_files()
        .await;

    if let Some(file_handles) = files {
        let paths: Vec<String> = file_handles.into_iter().map(|h| h.path().to_string_lossy().to_string()).collect();
        let mut all_previews = Vec::new();
        let mut app_state = state.lock().unwrap();

        for path_str in paths {
            let image_data_vec = cropper::crop_preview(&path_str);
            let previews = exporter::generate_base64_previews(&image_data_vec);
            all_previews.extend(previews);

            // Create a single group for this source image and its crops
            let group = SourceImageGroup {
                source_path: path_str.clone(),
                name: std::path::Path::new(&path_str).file_stem().unwrap_or_default().to_string_lossy().to_string(),
                artist: String::from("Artist Name"),
                crops: image_data_vec,
            };

            // Add the entire group to the state
            app_state.image_groups.push(group);
        }
        Ok(all_previews)
    } else {
        Ok(Vec::new())
    }
}

/*

Ran whenever a photo in the GUI is deselected. Also allow for updating the photo as selected.

*/
#[tauri::command]
pub fn set_selected(group_index: usize, crop_index: usize, selected: bool, state: State<'_, Mutex<AppState>>) {
    let mut app_state = state.lock().unwrap();
    // Safely get the group, then the crop, and update its `selected` field
    if let Some(group) = app_state.image_groups.get_mut(group_index) {
        if let Some(crop) = group.crops.get_mut(crop_index) {
            crop.selected = selected;
        }
    }
}

#[tauri::command]
pub fn update_row_metadata(
    group_index: usize, 
    name: String, 
    artist: String, 
    state: State<'_, Mutex<AppState>>
) {
    let mut app_state = state.lock().unwrap();

    // Safely get the correct group and update its name and artist fields
    if let Some(group) = app_state.image_groups.get_mut(group_index) {
        group.name = name;
        group.artist = artist;
    }
}

#[tauri::command]
pub fn update_pack_metadata() {

}

/*

Final command called when window is opened

Init:
 - Takes in an export path
After: 
 - Writes all the images to the export directory with given data, and closes the program

*/
#[tauri::command]
pub async fn export_pack(export_path: String) {
    todo!()
}