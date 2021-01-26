use serde::Deserialize;

use crate::locations::BoardLocation;

#[derive(Debug, Deserialize)]
pub struct Property {
    pub location: BoardLocation,
    pub tile_cost: usize,
    pub charge: usize,
    pub house_cost: usize,
    pub hotel_cost: usize,
}

impl Property {
    #[allow(dead_code)]
    pub fn new(
        location: BoardLocation,
        tile_cost: usize,
        charge: usize,
        house_cost: usize,
        hotel_cost: usize,
    ) -> Self {
        Self {
            location,
            tile_cost,
            charge,
            house_cost,
            hotel_cost,
        }
    }
}
