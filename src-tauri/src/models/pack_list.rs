use serde::Serialize;
use rand::Rng;

use crate::models::image_data::ImageData;

#[derive(Serialize, Debug)]
pub struct PackList<T> {
    #[serde(skip)]
    pub pack_name: String,
    #[serde(rename = "$schema")]
    pub schema: String,
    pub version: String, 
    pub id: String, 
    pub description: String,
    paintings: Vec<T>,
}

impl<T> Default for PackList<T> {
    fn default() -> Self {

        let mut rng = rand::rng();
        let random_int: i32 = rng.random_range(56000..=128000);
        let random_id = format!("{}", random_int);
        
        PackList {
            pack_name: String::from("Default"),
            schema: String::from("http://json-schema.org/draft-07/schema#"),
            version: String::from("1.0.0"),
            id: random_id, 
            description: String::from("A list of paintings in the gallery"),
            paintings: Vec::new(),
        }
    }
}

fn check_no_input(input: String) -> Option<String> {
    if input.trim().is_empty() {
        None
    } else {
         Some(input)
    }
}

impl<T> PackList<T> {

    pub fn set_schema(&mut self, schema: String) {
        match check_no_input(schema) {
            Some(valid_schema) => self.schema = valid_schema,
            None => {},
        }
    }

    pub fn set_version(&mut self, version: String) {
        match check_no_input(version) {
            Some(valid_version) => self.version = valid_version,
            None => {},
        }
    }

    pub fn set_id(&mut self, id: String) {
        match check_no_input(id) {
            Some(valid_id) => self.id = valid_id,
            None => {},
        }
    }

    pub fn set_description(&mut self, description: String) {
        match check_no_input(description) {
            Some(valid_description) => self.description = valid_description,
            None => {},
        }
    }

    pub fn add_painting(&mut self, painting: T) {
        self.paintings.push(painting);
    }

    pub fn separate_paintings<U>(self) -> (PackList<U>, Vec<T>) {
        
        // 1. Create the new struct with a new, empty `paintings` vector.
        //    This moves all the metadata fields (schema, id, etc.).
        let new_list = PackList {
            pack_name: self.pack_name,
            schema: self.schema,
            version: self.version,
            id: self.id,
            description: self.description,
            paintings: Vec::new(),
        };

        // 2. The only thing left in `self` is the original `paintings` vector.
        //    We can now move it out.
        let original_paintings = self.paintings;

        // 3. Return both new pieces as a tuple.
        (new_list, original_paintings)
    }
}

impl PackList<ImageData> {
    
    pub fn set_selected(&mut self, index: usize, selected: bool) {
        self.paintings[index].set_selected(selected);
    }

}