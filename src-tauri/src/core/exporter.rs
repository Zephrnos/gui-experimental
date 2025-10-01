use std::fs::{create_dir_all, write};
use image::{DynamicImage, ImageFormat};
use serde::Serialize;
use std::io::Cursor;
use base64::{Engine as _, engine::general_purpose};
use crate::models::pack_list::PackList;
use crate::models::image_data::ImageData;

// Load in the default icon to bianary so the file is contained in the executable
const DEFAULT_ICON: &[u8] = include_bytes!("../../assets/icon.png");

#[derive(Serialize)]
struct Painting {
    id:         String,
    filename:   String,
    name:       String,
    artist:     String,
    width:      u32,
    height:     u32
}

fn write_icon(export_path: &str) {
    create_dir_all(format!("{}/images", export_path)).expect("Failed to create images directory");
    write(format!("{}/icon.png", export_path), DEFAULT_ICON).expect("Failed to write default icon");
}

fn write_json (painting_list: &PackList<Painting>, export_path: &str) {
    let json_data = serde_json::to_string_pretty(painting_list).expect("Failed to serialize painting list");
    write(format!("{}/custompaintings.json", export_path), json_data).expect("Failed to write painting list JSON file");
}

fn write_images(painting_list: &mut PackList<Painting>, image_list: Vec<ImageData>, export_path: &str) {
    
    for image in image_list {

        if image.selected {

            let painting: DynamicImage = image.get_image().clone();

            for (width, height) in image.get_sizes() {

                let id: String = format!("{}_{}x{}", image.id.as_ref().unwrap(), &width, &height);
                let filename: String = format!("{}_{}x{}", image.filename.as_ref().unwrap(), &width, &height);
                painting.save(format!("{}/{}.png", export_path, &filename)).expect("This shouldnt fail");

                let painting: Painting = Painting {
                    id,
                    filename,
                    name: image.name.clone().unwrap(),
                    artist: image.artist.clone().unwrap(), 
                    width: *width, 
                    height: *height, 
                };

            painting_list.add_painting(painting);
                
            };
        }
    }
}


/*

This is a final export call to take all the files that we want and write them to a directory of our choice

*/
pub fn export(image_list: PackList<ImageData>, export_path: &str) {
    
    let (mut painting_list, image_data): (PackList<Painting>, Vec<ImageData>) =
        image_list.separate_paintings();

    write_images(&mut painting_list, image_data, export_path);
    write_json(&painting_list, export_path);
    write_icon(export_path);

}

/*

This creates Base64 previews of all images in a Vec<ImageData> for a local Tauri application.

Process:

Init:
 - Take in &Vec<ImageData>
Process:
 - For each image, write its PNG data to an in-memory buffer.
 - Encode the buffered data into a Base64 string.
 - Format the string as a "Data URI" (e.g., "data:image/png;base64,...").
After:
 - Return a Vec<String> of these Data URIs, which can be used directly in <img> src attributes.

*/
pub fn generate_base64_previews(image_list: &Vec<ImageData>) -> Vec<String> {
    let mut base64_images = Vec::new(); // Create a vector to store the Base64 strings

    for image in image_list {
        let preview_image = image.get_image();
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