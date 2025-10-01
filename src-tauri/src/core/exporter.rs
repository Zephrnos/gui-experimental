use std::fs::{create_dir_all, write};
use image::DynamicImage;
use serde::Serialize;
use crate::models::painting_list::PaintingList;
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

fn write_json (painting_list: &PaintingList<Painting>, export_path: &str) {
    let json_data = serde_json::to_string_pretty(painting_list).expect("Failed to serialize painting list");
    write(format!("{}/custompaintings.json", export_path), json_data).expect("Failed to write painting list JSON file");
}

fn write_images(painting_list: &mut PaintingList<Painting>, image_list: Vec<ImageData>, export_path: &str) {

    let mut index: usize = 0;
    
    for image in image_list {

        if painting_list.writable[index] {

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
        index += 1;
    }
}


/*

This is a final export call to take all the files that we want and write them to a directory of our choice

*/
pub fn export(image_list: PaintingList<ImageData>, export_path: &str) {
    
    let (mut painting_list, image_data): (PaintingList<Painting>, Vec<ImageData>) =
        image_list.separate_paintings();

    write_images(&mut painting_list, image_data, export_path);
    write_json(&painting_list, export_path);
    write_icon(export_path);

}

/*

This creates previews of all images in a Vec<ImageData>

Process:

Init:
 - Take in &Vec<ImageData>
 - Process the 5 views of the image as defined in image_size.rs
    - Square
    - Wide
    - LongRectangle
    - Tall
    - TallRectangle
 - Return a Vec<String>, where the String is the absolute name of the filepath to the image.

 ? Consider makign it return a Base64 String insetad so it is immediately readable by frontend HTML?

*/
pub fn save_previews(image_list: &Vec<ImageData>, dir: &str) -> Vec<String> {
    let mut saved_paths = Vec::new(); // Create a vector to store the paths

    for image in image_list {
        let preview_image = image.get_image();
        let file_path_str = format!("{}/{}.png", dir, image.filename.as_ref().unwrap());
        
        preview_image.save(&file_path_str).expect("This shouldn't fail");
        
        saved_paths.push(file_path_str); // Add the new path to our vector
    }

    saved_paths // Return the list of paths
}