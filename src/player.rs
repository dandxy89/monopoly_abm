use std::cell::RefCell;

use ring_buffer::RingBuffer;

use crate::token::Token;

#[allow(dead_code)]
pub struct PlayerState {
    pub current_position: usize,
    pub jail: Option<usize>,
    pub balance: i32,
}

#[allow(dead_code)]
pub struct Player {
    pub id: usize,
    pub token: Token,
    pub state: RefCell<PlayerState>,
}

impl Player {
    #[allow(dead_code)]
    pub fn create_players(n_players: usize) -> RingBuffer<Player> {
        let mut players = RingBuffer::with_capacity(n_players);
        for identity in 1..=n_players {
            players.push(Player {
                id: identity,
                token: Token::value_to_enum(identity),
                state: RefCell::new(PlayerState {
                    current_position: 0,
                    jail: None,
                    balance: 1500,
                }),
            });
        }

        players
    }
}

#[cfg(test)]
mod test {
    use super::{Player, Token};

    #[test]
    fn test_create_players() {
        let players = Player::create_players(3);
        let player_one = players.get_absolute(0).unwrap();
        let player_two = players.get_absolute(1).unwrap();
        let player_three = players.get_absolute(2).unwrap();

        assert!(player_one.id == 1 && player_one.token == Token::Iron);
        assert!(player_two.id == 2 && player_two.token == Token::Terrier);
        assert!(player_three.id == 3 && player_three.token == Token::Horse);
    }

    #[test]
    fn test_create_max() {
        let players = Player::create_players(10);
        let last_player = players.get_absolute(9).unwrap();

        assert!(last_player.id == 10 && last_player.token == Token::Boot);
    }
}
