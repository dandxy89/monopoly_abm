use std::cell::RefCell;

use ring_buffer::RingBuffer;

use crate::token::Token;

#[allow(dead_code)]
pub struct PlayerState {
    pub current_position: usize,
    pub jail: Option<usize>,
    pub balance: i32,
    pub active: bool,
}

#[allow(dead_code)]
pub struct Player {
    pub id: usize,
    pub token: Token,
    pub state: RefCell<PlayerState>,
}

impl Player {
    #[allow(dead_code)]
    pub fn bankrupt(&self) {
        let mut m = self.state.borrow_mut();
        m.active = false;
        m.balance = 0;
        m.jail = None;
    }

    #[allow(dead_code)]
    pub fn is_active(&self) -> bool {
        let s = self.state.borrow();
        s.active
    }

    #[allow(dead_code)]
    pub fn current_balance(&self) -> i32 {
        let s = self.state.borrow();
        s.balance
    }

    #[allow(dead_code)]
    fn can_afford(&self, charge: i32) -> bool {
        let b = self.state.borrow().balance;
        b - charge > 0
    }

    #[allow(dead_code)]
    pub fn pay(&self, charge: i32) -> i32 {
        let mut m = self.state.borrow_mut();
        if self.can_afford(charge) {
            m.balance -= charge;
            0
        } else {
            m.active = false;
            m.balance
        }
    }

    #[allow(dead_code)]
    pub fn go_to_jail(&self) {
        let mut m = self.state.borrow_mut();
        m.jail = Some(0);
    }

    #[allow(dead_code)]
    pub fn in_jail(&self) -> bool {
        self.state.borrow().jail.is_some()
    }

    #[allow(dead_code)]
    pub fn update_jail(&self, is_double: bool, bail: i32) {
        let mut m = self.state.borrow_mut();
        m.jail = m.jail.map(|v| v + 1);

        if is_double {
            m.jail = None;
        } else {
            match m.jail {
                Some(1) | Some(2) => (),
                _ => {
                    self.pay(bail);
                }
            }
        }
    }

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
                    active: true,
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
