use serde::{Serialize, Deserialize};


pub type Coordinates = std::vec::Vec<(f32, f32)>;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Stroke {
    pub coordinates: Coordinates,
    pub brush_size: usize,
    pub color: String,
}

pub type Drawing = std::vec::Vec<Stroke>;