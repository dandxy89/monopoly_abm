use std::{cell::RefCell, collections::HashMap};

use crate::locations::BoardLocationName;
use crate::player::{Player, PlayerId};

pub type BoardPosition = usize;

#[allow(dead_code)]
pub struct PropertyState {
    pub ownable: bool,
    pub owner: Option<PlayerId>,
    pub house_count: usize,
    pub hotel_count: usize,
    pub mortgaged: bool,
}

impl PropertyState {
    #[allow(dead_code)]
    pub fn new(square: &BoardLocationName) -> Self {
        match square {
            BoardLocationName::Go
            | BoardLocationName::IncomeTax
            | BoardLocationName::Chance1
            | BoardLocationName::Chance2
            | BoardLocationName::Chance3
            | BoardLocationName::CommunityChest1
            | BoardLocationName::CommunityChest2
            | BoardLocationName::Jail
            | BoardLocationName::FreeParking
            | BoardLocationName::GoToJail => Self {
                ownable: false,
                owner: None,
                house_count: 0,
                hotel_count: 0,
                mortgaged: false,
            },
            _ => Self {
                ownable: true,
                owner: None,
                house_count: 0,
                hotel_count: 0,
                mortgaged: false,
            },
        }
    }
}

pub type TileCost = usize;
pub type Rent = usize;
pub type HouseCost = usize;
pub type HotelCost = usize;

#[allow(dead_code)]
pub struct BoardSquare {
    position_id: BoardPosition,
    square: BoardLocationName,
    cost: usize,
    charge: usize,
    house_cost: usize,
    hotel_cost: usize,
    state: RefCell<PropertyState>,
}

impl BoardSquare {
    #[allow(dead_code)]
    pub fn new(
        location: BoardLocationName,
        location_config: &HashMap<BoardLocationName, (TileCost, Rent, HouseCost, HotelCost)>,
    ) -> Self {
        let (tile_cost, charge, house_cost, hotel_cost) = location_config.get(&location).unwrap();

        Self {
            position_id: 1,
            state: RefCell::new(PropertyState::new(&location)),
            cost: *tile_cost,
            square: location,
            charge: *charge,
            house_cost: *house_cost,
            hotel_cost: *hotel_cost,
        }
    }

    #[allow(dead_code)]
    pub fn is_ownable(&self) -> bool {
        let s = self.state.borrow();
        s.ownable
    }

    #[allow(dead_code)]
    pub fn is_owned(&self) -> bool {
        let s = self.state.borrow();
        s.owner.is_some()
    }

    #[allow(dead_code)]
    pub fn get_purchase_cost(&self) -> usize {
        self.cost
    }

    #[allow(dead_code)]
    pub fn rent_cost(&self) -> usize {
        let s = self.state.borrow();
        println!("{}-{}", s.house_count, s.hotel_count);
        if s.hotel_count + s.house_count == 0 {
            self.charge
        } else if s.hotel_count < 1 {
            s.house_count * self.charge + self.charge
        } else {
            s.hotel_count * 5 * self.charge + self.charge
        }
    }

    #[allow(dead_code)]
    pub fn purchase_property(&self, player: &Player) {
        let mut s = self.state.borrow_mut();
        s.owner = Some(player.id);
        player.pay(self.cost);
    }

    #[allow(dead_code)]
    pub fn upgradable(&self) -> bool {
        let s = self.state.borrow();
        s.hotel_count < 2 && s.house_count <= 4
    }

    #[allow(dead_code)]
    pub fn upgrade_cost(&self) -> usize {
        let s = self.state.borrow();
        if s.house_count <= 4 {
            self.house_cost
        } else if s.hotel_count == 1 {
            self.hotel_cost
        } else {
            0
        }
    }

    fn upgrade(&self) {
        let mut s = self.state.borrow_mut();
        if s.house_count < 4 {
            s.house_count += 1;
        } else if s.hotel_count == 0 {
            s.hotel_count += 1;
        }
    }

    #[allow(dead_code)]
    pub fn purchase_upgrade(&self, player: &Player) {
        player.pay(self.upgrade_cost());

        if player.is_active() {
            self.upgrade();
        }
    }

    // TODO Community Chest | Chance
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use crate::player::Player;

    use super::{BoardLocationName, BoardSquare};

    #[test]
    fn test_property_is_ownable() {
        let mut config = HashMap::new();
        config.insert(BoardLocationName::Piccadilly, (300, 50, 100, 200));

        let sq = BoardSquare::new(BoardLocationName::Piccadilly, &config);
        assert!(sq.is_ownable());
    }

    #[test]
    fn test_property_is_not_ownable() {
        let mut config = HashMap::new();
        config.insert(BoardLocationName::Go, (0, 0, 0, 0));

        let sq = BoardSquare::new(BoardLocationName::Go, &config);
        assert!(!sq.is_ownable());
    }

    #[test]
    fn test_get_cost() {
        let mut config = HashMap::new();
        config.insert(BoardLocationName::Piccadilly, (300, 50, 100, 200));

        let sq = BoardSquare::new(BoardLocationName::Piccadilly, &config);
        let player_one = Player::new(1);

        sq.purchase_property(&player_one);
        assert!(sq.is_owned());

        // No Houses & no hotels
        assert_eq!(sq.rent_cost(), 50);

        // 1 Houses & no hotels
        assert_eq!(sq.upgrade_cost(), 100);
        sq.purchase_upgrade(&player_one);
        assert_eq!(sq.rent_cost(), 100);

        // 2 Houses & no hotels
        assert_eq!(sq.upgrade_cost(), 100);
        sq.purchase_upgrade(&player_one);
        assert_eq!(sq.rent_cost(), 150);

        // 3 Houses & no hotels
        assert_eq!(sq.upgrade_cost(), 100);
        sq.purchase_upgrade(&player_one);
        assert_eq!(sq.rent_cost(), 200);

        // 4 Houses & no hotels
        assert_eq!(sq.upgrade_cost(), 100);
        sq.purchase_upgrade(&player_one);
        assert_eq!(sq.rent_cost(), 250);

        // 4 Houses & 1 hotels
        assert_eq!(sq.upgrade_cost(), 100);
        sq.purchase_upgrade(&player_one);
        assert_eq!(sq.rent_cost(), 300);
    }
}
