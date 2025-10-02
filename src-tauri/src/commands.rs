use std::sync::Mutex;
use tauri::{State, Window, Emitter};
use crate::{
    app_state::{AppState, SourceImageGroup}, 
    core::{cropper, exporter},
    models::{image_data::ImageData, image_size::ImageSize},
    core::exporter::ExportItem,
};

// Payload for the event emitted after each image is processed.
#[derive(Clone, serde::Serialize)]
struct ImageProcessedPayload {
    previews: Vec<String>,
    name: String,
    artist: String,
}

/*
Test Command
 */
#[tauri::command]
pub fn my_custom_command() {
  println!("I was invoked from JavaScript!");
}

/*
Opens images, generates transient crops, and emits an event for each image
with its Base64 previews. This avoids accumulating all previews in memory
and sending a single large payload.
*/
#[tauri::command]
pub async fn open_and_process_images(state: State<'_, Mutex<AppState>>, window: Window) -> Result<(), String> {
    println!("[COMMAND] open_and_process_images command received commands.rs");
    let files = rfd::AsyncFileDialog::new()
        .set_title("Choose Images...")
        .add_filter("Image Files", &["png", "jpg", "jpeg"])
        .pick_files()
        .await;
    println!("[COMMAND] open_and_process_images images received commands.rs");

    if let Some(file_handles) = files {
        let paths: Vec<String> = file_handles.into_iter().map(|h| h.path().to_string_lossy().to_string()).collect();
        
        // The AppState is locked once outside the loop for efficiency.
        let mut app_state = state.lock().unwrap();

        for path_str in paths {
            // 1. Generate cropped images in memory (transiently).
            let cropped_images = match cropper::generate_cropped_images(&path_str) {
                Ok(images) => images,
                Err(e) => {
                    eprintln!("Failed to crop image {}: {}", path_str, e);
                    continue; // Skip this image if it fails to open/crop
                }
            };
            println!("[COMMAND] open_and_process_images image cropped commands.rs");
            
            // 2. Create Base64 previews from the transient images.
            let previews = exporter::generate_base64_previews(&cropped_images);
            println!("[COMMAND] open_and_process_images image converted base64 commands.rs");
            
            // 3. Create metadata-only ImageData structs for the app state.
            let crop_metadata: Vec<ImageData> = ImageSize::iter()
                .map(|size_variant| ImageData::new(*size_variant))
                .collect();
            
            let name = std::path::Path::new(&path_str).file_stem().unwrap_or_default().to_string_lossy().to_string();
            let artist = String::from("Artist Name");

            // 4. EMIT an event with the previews and initial metadata for THIS image group.
            // The frontend will listen for this and build the UI row by row.
            window.emit("image-processed", ImageProcessedPayload {
                previews: previews.clone(),
                name: name.clone(),
                artist: artist.clone(),
            }).unwrap();

            // 5. Create the group with the source path and metadata, then store in state.
            let group = SourceImageGroup {
                source_path: path_str.clone(),
                name,
                artist,
                crops: crop_metadata,
            };
            app_state.image_groups.push(group);

            // `cropped_images` is dropped here, freeing its memory.
        }
        
        // After the loop, emit a final event to signal completion.
        window.emit("processing-finished", ()).unwrap();

    } else {
        // If the user cancelled the dialog, we still emit the finished event
        // to ensure the frontend doesn't get stuck in a loading state.
        window.emit("processing-finished", ()).unwrap();
    }
    
    Ok(())
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
Collects all metadata and source paths, then passes them to the exporter,
which re-opens and re-crops images on-demand.
*/
#[tauri::command]
pub async fn export_pack(state: State<'_, Mutex<AppState>>) -> Result<(), String> {
    println!("[COMMAND] export_pack received commands.rs");

    // 1. Open a native dialog to have the user pick the export directory
    let folder = rfd::AsyncFileDialog::new()
        .set_title("Choose Export Directory...")
        .pick_folder()
        .await;

    // 2. Only proceed if the user selected a folder (didn't cancel)
    if let Some(folder_handle) = folder {
        let export_path = folder_handle.path().to_string_lossy().to_string();
        let app_state = state.lock().unwrap();

        // 3. Create a list of items to be exported, including source paths for re-cropping.
        let mut items_to_export: Vec<ExportItem> = Vec::new();
        for group in &app_state.image_groups {
            for crop in &group.crops {
                if crop.selected { // Check if the crop is selected
                    let mut export_crop_data = crop.clone();
                    // Assign the shared metadata from the group to the individual crop
                    export_crop_data.name = Some(group.name.clone());
                    export_crop_data.artist = Some(group.artist.clone());
                    export_crop_data.id = Some(group.name.clone());
                    export_crop_data.filename = Some(group.name.clone());
                    
                    items_to_export.push(ExportItem {
                        source_path: group.source_path.clone(),
                        data: export_crop_data,
                    });
                }
            }
        }

        // 4. Call the exporter with the raw metadata and the list of items to process.
        // This module no longer needs to know about the private `Painting` struct.
        let pack_meta = &app_state.pack_metadata;
        exporter::export(
            pack_meta.pack_name.clone(),
            pack_meta.version.clone(),
            pack_meta.id.clone(),
            pack_meta.description.clone(),
            items_to_export,
            &export_path,
        );
    }
    
    // If the user cancels the dialog, the function simply finishes without error.
    Ok(())
}
