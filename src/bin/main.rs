use std::vec;

use monopoly_abm::monopoly::MonopolyState;
use monopoly_abm::player::Player;

fn main() {
    let n_players: usize = 3;
    let _players = Player::create_players(n_players);
    let _game_state = MonopolyState::new(vec![]);
    // players.iter().collect()
}
