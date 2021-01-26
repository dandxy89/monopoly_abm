use std::cell::RefCell;
use std::collections::HashMap;

use crate::player::{Player, PlayerId};
use crate::state::State;
use crate::{config::PropertyConfig, locations::BoardLocation};

pub struct Count(usize);

#[allow(dead_code)]
pub struct MonopolyState {
    free_parking: RefCell<Count>,
    player_position: RefCell<HashMap<PlayerId, BoardLocation>>,
    cycles: RefCell<Count>,
}

impl MonopolyState {
    #[allow(dead_code)]
    pub fn new(
        n_players: usize,
        _property_config: Vec<PropertyConfig>,
        _players: Vec<&Player>,
    ) -> Self {
        let mut positions = HashMap::new();
        for p_id in 1..=n_players {
            positions.insert(p_id, BoardLocation::Go);
        }

        Self {
            free_parking: RefCell::new(Count(0)),
            player_position: RefCell::new(positions),
            cycles: RefCell::new(Count(0)),
        }
    }
}

impl State for MonopolyState {
    fn update(&self) {
        self.cycles.borrow_mut().0 += 1;
        log::info!("Cycle [{}] complete", self.cycles.borrow().0);
    }
}
