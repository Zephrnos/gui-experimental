#[derive(Debug)]
pub struct PaintingList<T> {
    // #[serde(rename = "$schema")]
    pub schema: String,
    pub version: String, 
    pub id: String, 
    pub description: String,
    // #[serde(skip)]
    to_write: Vec<bool>,
    paintings: Vec<T>,
}

impl<T> Default for PaintingList<T> {
    fn default() -> Self {
        PaintingList {
            schema: String::from("http://json-schema.org/draft-07/schema#"),
            version: String::from("1.0.0"),
            id: String::from("http://example.com/paintinglist.json"),
            description: String::from("A list of paintings in the gallery"),
            to_write: Vec::new(),
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

impl<T> PaintingList<T> {

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

    pub fn add_many_paintings(&mut self, paintings: Vec<T>) {
        for painting in paintings {
            self.paintings.push(painting);
        }
    }

    pub fn retrieve_paintings(&self) -> &Vec<T> {
        &self.paintings
    }

}