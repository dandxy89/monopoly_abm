use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PropertyConfig {
    pub tile_cost: usize,
    pub charge: usize,
    pub house_cost: usize,
    pub hotel_cost: usize,
}

impl PropertyConfig {
    #[allow(dead_code)]
    pub fn new(tile_cost: usize, charge: usize, house_cost: usize, hotel_cost: usize) -> Self {
        Self {
            tile_cost,
            charge,
            house_cost,
            hotel_cost,
        }
    }
}
