use std::cell::RefCell;

use rand::prelude::ThreadRng;

use crate::config::Property;
use crate::state::State;

#[derive(Debug)]
pub struct Count(usize);

#[derive(Debug)]
pub struct MonopolyState {
    pub free_parking: RefCell<Count>,
    cycles: RefCell<Count>,
    pub rng: ThreadRng,
}

impl MonopolyState {
    pub fn new(_property_config: Vec<Property>) -> Self {
        Self {
            free_parking: RefCell::new(Count(0)),
            cycles: RefCell::new(Count(0)),
            rng: ThreadRng::default(),
        }
    }
}

impl<'a> State for MonopolyState {
    fn update(&self) {
        self.cycles.borrow_mut().0 += 1;
        log::info!("Cycle [{}] complete", self.cycles.borrow().0);
    }
}
