use ring_buffer::RingBuffer;

use crate::player::Player;

#[allow(dead_code)]
pub struct Monopoly {
    players: RingBuffer<Player>,
}

impl Monopoly {
    #[allow(dead_code)]
    fn new(n_players: usize) -> Self {
        Self {
            players: Player::create_players(n_players),
        }
    }
}
