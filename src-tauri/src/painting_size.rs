use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum PaintingSize {
    Square,
    Wide, 
    LongRectangle,
    Tall,
    TallRectangle,
}

impl PaintingSize {

    pub fn get_size(&self) -> Vec<(u32, u32)> {
        match self {
            PaintingSize::Square => vec![(1, 1), (2, 2), (3, 3), (4, 4)],
            PaintingSize::Wide => vec![(2, 1), (4, 2)],
            PaintingSize::LongRectangle => vec![(4, 3)],
            PaintingSize::Tall => vec![(1, 2), (2, 4)],
            PaintingSize::TallRectangle => vec![(3, 4)],
        }
    }

}
