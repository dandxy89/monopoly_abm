use std::{cell::RefCell, collections::HashMap};

use rand::rngs::ThreadRng;

use crate::locations::BoardLocationName;
use crate::payment::Payment;
use crate::player::{Player, PlayerId};

pub type BoardPosition = usize;
pub type Movement = usize;
pub type FreeParking = usize;

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
    pub fn owner_id(&self) -> Option<usize> {
        let s = self.state.borrow();
        s.owner
    }

    #[allow(dead_code)]
    pub fn is_owned_by_player(&self, player: &Player) -> bool {
        match self.state.borrow().owner {
            Some(id) => player.id == id,
            None => false,
        }
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

    // Community Chest
    // - Advance to Go
    // - Banking error collect 200
    // - Doctors Fees pay 50
    // - Sale from Stock collect 50
    // - Get out of Jail free
    // - Holiday Xmas func matures collect 100
    // - Income Tax refund collect 20
    // - Its your birthday collect 10 from each player
    // - Life Insurance matures collect 100
    // - Hospital fees pay 50
    // - School fees pay 50
    // - Street repairs 40 per house and 115 per hotel
    // - Second play in a Beauty competition collect 20
    // - You inheritance comes through collect 100

    // Chance
    // - Advance to Go
    // - Go to jail
    // - Advance to Pall Mall. If you pass "Go" collection £200
    // - Take a trip to Marylebone Station and if you pass "Go" collect £200
    // - Advance to Trafalgar Square. If you pass "Go" collect £200
    // - Advance to Mayfair
    // - Go back three spaces
    // - Make general repairs on all of your houses. For each house pay £25. For each hotel pay £100
    // - You are assessed for street repairs: £40 per house, £115 per hotel
    // - Pay school fees of £150
    // - "Drunk in charge" fine £20
    // - Speeding fine £15
    // - Your building loan matures. Receive £150
    // - You have won a crossword competition. Collect £100
    // - Bank pays you dividend of £50
    // - Get out of jail free. This card may be kept until needed or sold

    #[allow(dead_code)]
    pub fn take_step(
        &self,
        new_position: BoardSquare,
        player: &Player,
        _rng: &mut ThreadRng,
    ) -> (Movement, Option<Payment>, FreeParking) {
        match new_position.square {
            BoardLocationName::Go => {
                player.deposit(200);
                (0, None, 0)
            }
            BoardLocationName::IncomeTax => {
                player.pay(200); // OR 10% of the value of the Assets
                (0, None, 0)
            }
            BoardLocationName::LuxuryTax => {
                player.pay(75);
                (0, None, 0)
            }
            BoardLocationName::Chance1
            | BoardLocationName::Chance2
            | BoardLocationName::Chance3 => {
                // RANDOMLY SELECT CARD
                unimplemented!()
            }
            BoardLocationName::CommunityChest1 | BoardLocationName::CommunityChest2 => {
                // RANDOMLY SELECT CARD
                unimplemented!()
            }
            BoardLocationName::GoToJail => {
                player.go_to_jail();
                (9, None, 0)
            }
            BoardLocationName::Jail => (0, None, 0), // Just Visiting
            _ => {
                // Not bought
                if self.is_ownable() && player.can_afford(self.get_purchase_cost()) {
                    self.purchase_property(player);
                    return (0, None, 0);
                }

                // Owned by Player - maybe an upgrade?
                let rent = self.rent_cost();
                if self.is_owned_by_player(player) && player.can_afford(self.upgrade_cost()) {
                    self.purchase_upgrade(player);
                    (0, None, 0)
                } else if player.can_afford(rent) {
                    player.pay(rent);

                    (
                        0,
                        Some(Payment {
                            to: self.owner_id().unwrap(),
                            amount: rent,
                            terminal: false,
                        }),
                        0,
                    )
                } else {
                    (
                        0,
                        Some(Payment {
                            to: self.owner_id().unwrap(),
                            amount: rent,
                            terminal: true,
                        }),
                        0,
                    )
                }
            }
        }
    }
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
