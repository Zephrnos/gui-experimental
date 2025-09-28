use serde::{Serialize};
use std::slice::Iter;

#[derive(Serialize, Debug, Clone, Copy)]
pub enum ImageSize {
    Square,
    Wide, 
    LongRectangle,
    Tall,
    TallRectangle,
}

impl ImageSize {

    pub fn iter() -> Iter<'static, ImageSize> {
        static PAINTING_SIZES: [ImageSize; 5] = [
            ImageSize::Square,
            ImageSize::Wide,
            ImageSize::LongRectangle,
            ImageSize::Tall,
            ImageSize::TallRectangle,
        ];
        PAINTING_SIZES.iter()
    }

    pub fn get_size(&self) -> Vec<(u32, u32)> {
        match self {
            ImageSize::Square => vec![(1, 1), (2, 2), (3, 3), (4, 4)],
            ImageSize::Wide => vec![(2, 1), (4, 2)],
            ImageSize::LongRectangle => vec![(4, 3)],
            ImageSize::Tall => vec![(1, 2), (2, 4)],
            ImageSize::TallRectangle => vec![(3, 4)],
        }
    }

}
