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

fn write_icon(export_path: &String) {
    create_dir_all(format!("{}/images", export_path)).expect("Failed to create images directory");
    write(format!("{}/icon.png", export_path), DEFAULT_ICON).expect("Failed to write default icon");
}

fn write_json (painting_list: &PaintingList<Painting>, export_path: &String) {
    let json_data = serde_json::to_string_pretty(painting_list).expect("Failed to serialize painting list");
    write(format!("{}/custompaintings.json", export_path), json_data).expect("Failed to write painting list JSON file");
}

fn write_images(painting_list: &mut PaintingList<Painting>, image_list: Vec<ImageData>, export_path: &String) {
    
    for image in image_list {

        for (width, height) in image.get_sizes() {

            let id: String = format!("{}_{}x{}", image.id.clone().unwrap(), &width, &height);
            let filename: String = format!("{}_{}x{}", image.filename.clone().unwrap(), &width, &height);
            let painting: DynamicImage = image.get_image().clone();
            painting.save(format!("{}/{}.png", export_path, &filename)).expect("This shouldnt fail");

            let painting: Painting = Painting {
                id,
                filename,
                name: image.name.clone().unwrap(),
                artist: image.artist.clone().unwrap(), 
                width, 
                height, 
            };

        painting_list.add_painting(painting);
            
        };

    }

}

pub fn export(image_list: PaintingList<ImageData>, export_path: String) {
    

    let (mut painting_list, image_data): (PaintingList<Painting>, Vec<ImageData>) =
        image_list.separate_paintings();

    write_images(&mut painting_list, image_data, &export_path);
    write_json(&painting_list, &export_path);
    write_icon(&export_path);



}

