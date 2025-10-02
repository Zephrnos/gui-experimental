use std::fs::{create_dir_all, write};
use image::{DynamicImage, ImageFormat};
use serde::Serialize;
use std::io::Cursor;
use base64::{Engine as _, engine::general_purpose};
use crate::models::pack_list::PackList;
use crate::models::image_data::ImageData;
use crate::core::cropper;

// Load in the default icon to bianary so the file is contained in the executable
const DEFAULT_ICON: &[u8] = include_bytes!("../../assets/icon.png");

// The Painting struct is now private to this module.
#[derive(Serialize)]
struct Painting {
    id:         String,
    filename:   String,
    name:       String,
    artist:     String,
    width:      u32,
    height:     u32
}

/*
This creates Base64 previews from a Vec<DynamicImage> for the Tauri frontend.
The images are passed in directly and are not retrieved from app state.
*/
pub fn generate_base64_previews(image_list: &Vec<DynamicImage>) -> Vec<String> {
    let mut base64_images = Vec::new(); // Create a vector to store the Base64 strings

    for preview_image in image_list {
        let mut image_buffer: Vec<u8> = Vec::new();

        // Write the image's PNG data into our in-memory buffer
        preview_image.write_to(
            &mut Cursor::new(&mut image_buffer),
            ImageFormat::Png,
        ).expect("Failed to write image to buffer");
        
        // Encode the binary data into a Base64 string
        let base64_string = general_purpose::STANDARD.encode(&image_buffer);
        
        // Format the string as a Data URI and add it to our vector
        base64_images.push(format!("data:image/png;base64,{}", base64_string));
    }

    base64_images // Return the list of Data URIs
}

fn write_icon(export_path: &str) {
    write(format!("{}/icon.png", export_path), DEFAULT_ICON).expect("Failed to write default icon");
}
fn write_json (painting_list: &PackList<Painting>, export_path: &str) {
    let json_data = serde_json::to_string_pretty(painting_list).expect("Failed to serialize painting list");
    write(format!("{}/custompaintings.json", export_path), json_data).expect("Failed to write painting list JSON file");
}

// This new struct is used to package all necessary data for a single exportable image.
pub struct ExportItem {
    pub source_path: String,
    pub data: ImageData,
}


fn write_images(painting_list: &mut PackList<Painting>, image_list: Vec<ExportItem>, export_path: &str) {
    
    let images_dir = format!("{}/images", export_path);
    create_dir_all(&images_dir).expect("Failed to create images directory");

    for item in image_list {
        // Re-create the image from the source path on-demand for export and make it mutable.
        let mut painting = cropper::crop_single_image(&item.source_path, &item.data.image_size)
            .expect("Failed to re-crop image for export.");

        if painting.width() > 1024 {
            painting = painting.thumbnail(1024, u32::MAX);
        }

        for (width, height) in item.data.get_sizes() {

            let sanitized_id = item.data.id.as_ref().unwrap().replace(' ', "_");
            let sanitized_filename = item.data.filename.as_ref().unwrap().replace(' ', "_");

            let id: String = format!("{}_{}x{}", &sanitized_id, &width, &height);
            let base_filename: String = format!("{}_{}x{}", &sanitized_filename, &width, &height);
            
            let save_path = format!("{}/{}.png", &images_dir, &base_filename);
            painting.save(save_path).expect("This shouldnt fail");

            let painting_meta: Painting = Painting {
                id,
                filename: format!("{}.png", base_filename),
                name: item.data.name.clone().unwrap(),
                artist: item.data.artist.clone().unwrap(), 
                width: *width, 
                height: *height, 
            };
            painting_list.add_painting(painting_meta);
        };
    }
}


/*
This is the final export call. It now accepts the raw metadata components
and is responsible for creating the PackList<Painting> internally.
*/
pub fn export(
    pack_name: String,
    version: String,
    id: String,
    description: String,
    items_to_export: Vec<ExportItem>,
    export_path: &str,
) {
    // --- NEW: Sanitize Pack Name and ID ---
    // Sanitize the pack name for use in the directory path.
    let sanitized_pack_name = pack_name.replace(' ', "_");
    let pack_dir = format!("{}/{}", export_path, &sanitized_pack_name);

    let sanitized_pack_id: String = id
        .to_lowercase()
        .replace(' ', "_")
        .chars()
        .filter(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || *c == '_')
        .collect();

    let mut painting_list = PackList::new(
        pack_name,
        version,
        sanitized_pack_id,
        description,
    );

    write_images(&mut painting_list, items_to_export, &pack_dir);
    write_json(&painting_list, &pack_dir);
    write_icon(&pack_dir);
}