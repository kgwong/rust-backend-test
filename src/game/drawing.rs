use serde::{Serialize, Deserialize};


pub type Coordinates = std::vec::Vec<(i32, i32)>;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Stroke {
    pub coordinates: Coordinates,
    pub brush_size: usize,
    pub color: String,
}

pub type Drawing = std::vec::Vec<Stroke>;