use std::cell::RefCell;

use ring_buffer::RingBuffer;

use crate::{player::Player, state::State};

#[allow(dead_code)]
pub struct Monopoly {
    players: RingBuffer<Player>,
    free_parking: RefCell<usize>,
    // player_position: RefCell<HashMap<PlayerId, BoardPosition>>,
}

impl Monopoly {
    // #[allow(dead_code)]
    // fn new(n_players: usize) -> Self {
    //     Self {
    //         players: Player::create_players(n_players),
    //         free_parking: RefCell::new(0),
    //         // player_position
    //     }
    // }
}

impl State for Monopoly {
    fn update(&mut self) {
        todo!()
    }
}
