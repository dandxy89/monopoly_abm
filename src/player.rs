use std::cell::RefCell;

use crate::{agent::Agent, dice::roll_game_dice, monopoly::MonopolyState, token::Token};

pub type PlayerId = usize;

const JAIL_BAIL: usize = 50;

#[allow(dead_code)]
#[derive(Debug)]
pub struct PlayerState {
    pub current_position: usize,
    pub jail: Option<usize>,
    pub balance: usize,
    pub active: bool,
    pub go_count: usize,
    pub jail_count: usize,
    pub house_count: usize,
    pub hotel_count: usize,
    pub jail_card: bool,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Player {
    pub id: PlayerId,
    pub token: Token,
    pub state: RefCell<PlayerState>,
}

impl Player {
    #[must_use]
    pub const fn new(id: usize) -> Self {
        Self {
            id,
            token: Token::value_to_enum(id),
            state: RefCell::new(PlayerState {
                current_position: 0,
                jail: None,
                balance: 1500,
                active: true,
                go_count: 0,
                jail_count: 0,
                house_count: 0,
                hotel_count: 0,
                jail_card: false,
            }),
        }
    }

    pub fn current_position(&self) -> usize {
        let s = self.state.borrow();
        s.current_position
    }

    pub fn move_position(&self, steps: usize) -> usize {
        let mut s = self.state.borrow_mut();
        s.current_position += steps;
        s.current_position
    }

    pub fn jail_card(&self) {
        let mut s = self.state.borrow_mut();
        s.jail_card = true;
    }

    pub fn has_jail_card(&self) -> bool {
        let s = self.state.borrow();
        s.jail_card
    }

    pub fn add_house(&self) {
        let mut s = self.state.borrow_mut();
        s.house_count += 1;
    }

    pub fn add_hotel(&self) {
        let mut s = self.state.borrow_mut();
        s.hotel_count += 1;
    }

    pub fn count_properties(&self) -> (usize, usize) {
        let s = self.state.borrow();
        (s.house_count, s.hotel_count)
    }

    pub fn is_active(&self) -> bool {
        self.state.borrow().active
    }

    pub fn current_balance(&self) -> usize {
        self.state.borrow().balance
    }

    pub fn can_afford(&self, charge: usize) -> bool {
        self.state.borrow().balance - charge > 0
    }

    pub fn deposit(&self, value: usize) {
        let mut s = self.state.borrow_mut();
        s.balance += value;
    }

    pub fn pay(&self, charge: usize) {
        let mut s = self.state.borrow_mut();
        if s.balance as i32 - charge as i32 > 0 {
            s.balance -= charge;
        } else {
            s.active = false;
        }
    }

    #[allow(dead_code)]
    pub fn go_to_jail(&self) -> usize {
        let mut s = self.state.borrow_mut();
        let mut days = 1;
        s.jail = match s.jail {
            Some(v) => {
                days = v + 1;
                Some(days)
            }
            None => {
                s.jail_count += 1;
                Some(0)
            }
        };

        days
    }

    #[allow(dead_code)]
    pub fn in_jail(&self) -> bool {
        self.state.borrow().jail.is_some()
    }

    #[allow(dead_code)]
    pub fn get_out_of_jail(&self) {
        let mut s = self.state.borrow_mut();
        s.jail = None;
        s.jail_card = false;
    }

    #[allow(dead_code)]
    pub fn update_jail(&self, is_double: bool, bail: usize) {
        let count = self.go_to_jail();

        if is_double || self.has_jail_card() {
            self.get_out_of_jail();
        } else if count == 3 {
            self.pay(bail);
            self.get_out_of_jail();
        }
    }

    #[must_use]
    pub fn create_players(n_players: usize) -> Vec<Self> {
        let mut players = Vec::with_capacity(n_players);
        for identity in 1..=n_players {
            players.push(Self::new(identity));
        }

        players
    }
}

impl Agent for Player {
    type SimState = MonopolyState;

    fn step(&self, _state: &Self::SimState, _players: Vec<&Player>) {
        let mut r = rand::thread_rng();
        let mut roll_count = 0;

        if self.in_jail() && self.is_active() {
            let roll_result = roll_game_dice(&mut r);
            self.update_jail(roll_result.is_double, JAIL_BAIL);
            roll_count += 1;
        }

        if !self.in_jail() && self.is_active() {
            loop {
                let roll_result = roll_game_dice(&mut r);
                if roll_result.is_double {
                    roll_count += 1;
                    if roll_count == 3 {
                        self.go_to_jail();
                        break;
                    }
                }
                let _current_position = self.move_position(roll_result.value);

                if !roll_result.is_double {
                    break;
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::{Player, Token};

    #[test]
    fn test_create_players() {
        let players = Player::create_players(3);
        let player_one = &players[0];

        assert!(player_one.id == 1 && player_one.token == Token::Iron);
    }

    #[test]
    fn is_player_in_jail() {
        let gamer_one = Player::new(1);
        assert!(gamer_one.is_active());
        assert!(!gamer_one.in_jail());

        gamer_one.go_to_jail();
        assert!(gamer_one.in_jail());
    }

    #[test]
    fn purchasing_a_property() {
        let gamer_two = Player::new(2);
        assert!(gamer_two.is_active());
        assert!(gamer_two.can_afford(100));

        gamer_two.pay(1000);
        assert!(gamer_two.is_active());
        assert_eq!(gamer_two.current_balance(), 500);

        gamer_two.pay(1000);
        assert!(!gamer_two.is_active());
    }

    #[test]
    fn jail_sequence_with_double() {
        let gamer_three = Player::new(2);
        let balance = gamer_three.current_balance();
        assert!(!gamer_three.in_jail());

        gamer_three.go_to_jail();
        assert!(gamer_three.in_jail());

        gamer_three.update_jail(false, 30); // #1
        assert!(gamer_three.in_jail());

        gamer_three.update_jail(false, 30); // #2
        assert!(gamer_three.in_jail());

        gamer_three.update_jail(true, 30); // #3
        assert!(!gamer_three.in_jail());
        assert_eq!(gamer_three.current_balance(), balance);
    }

    #[test]
    fn jail_sequence_with_no_double() {
        let gamer_three = Player::new(2);
        let balance = gamer_three.current_balance();
        assert!(!gamer_three.in_jail());

        gamer_three.go_to_jail();
        assert!(gamer_three.in_jail());

        gamer_three.update_jail(false, 30); // #1
        assert!(gamer_three.in_jail());

        gamer_three.update_jail(false, 30); // #2
        assert!(gamer_three.in_jail());

        gamer_three.update_jail(false, 30); // #3
        assert!(!gamer_three.in_jail());
        assert_eq!(gamer_three.current_balance(), balance - 30);
    }
}
