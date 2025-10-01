use std::sync::Mutex;
use tauri::State;
use crate::{
    app_state::{AppState, SourceImageGroup}, 
    core::{cropper, exporter},
    models::{pack_list::PackList}
};

/*
Test Command
 */
#[tauri::command]
pub fn my_custom_command() {
  println!("I was invoked from JavaScript!");
}

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
    println!("[COMMAND] open_and_process_images received commands.rs");
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
    println!("[COMMAND] set_selected received commands.rs");
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
    println!("[COMMAND] update_row_metadata received commands.rs");
    let mut app_state = state.lock().unwrap();

    // Safely get the correct group and update its name and artist fields
    if let Some(group) = app_state.image_groups.get_mut(group_index) {
        group.name = name;
        group.artist = artist;
    }
}

#[tauri::command]
pub fn update_pack_metadata(
    pack_name: String,
    version: String,
    id: String,
    description: String,
    state: State<'_, Mutex<AppState>>
) {
    println!("[COMMAND] update_pack_metadata received commands.rs");
    let mut app_state = state.lock().unwrap();

    let pack_metadata = &mut app_state.pack_metadata;

    pack_metadata.set_pack_name(&pack_name);
    pack_metadata.set_version(&version);
    pack_metadata.set_id(&id);
    pack_metadata.set_description(&description);

}

/*

Final command called when window is opened

Init:
 - Takes in an export path
After: 
 - Writes all the images to the export directory with given data, and closes the program

*/
#[tauri::command]
pub async fn export_pack(export_path: String, state: State<'_, Mutex<AppState>>) -> Result<(), String> {
    println!("[COMMAND] export_pack received commands.rs");
    let app_state = state.lock().unwrap();

    // 1. Create the final list for export, starting with global metadata
    let mut final_list = PackList::new(
        app_state.pack_metadata.pack_name.clone(),
        app_state.pack_metadata.version.clone(),
        app_state.pack_metadata.id.clone(),
        app_state.pack_metadata.description.clone(),
    );

    // 2. Iterate through the groups and add only the selected crops
    for group in &app_state.image_groups {
        for crop in &group.crops {
            if crop.selected { // Check if the crop is selected
                let mut export_crop = crop.clone();
                // Assign the shared metadata from the group to the individual crop
                export_crop.name = Some(group.name.clone());
                export_crop.artist = Some(group.artist.clone());
                final_list.add_painting(export_crop);
            }
        }
    }

    // 3. Call the exporter with the fully prepared list
    exporter::export(final_list, &export_path);
    Ok(())
}