use rand::distributions::{Distribution, Uniform};
use std::{cell::RefCell, collections::HashMap};

use rand::rngs::ThreadRng;

use crate::config::Property;
use crate::locations::BoardLocation;
use crate::payment::Payment;
use crate::player::{Player, PlayerId};

pub type BoardPosition = usize;
pub type MoveTo = usize;
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
    pub const fn new(square: BoardLocation) -> Self {
        match square {
            BoardLocation::Go
            | BoardLocation::IncomeTax
            | BoardLocation::Chance1
            | BoardLocation::Chance2
            | BoardLocation::Chance3
            | BoardLocation::CommunityChest1
            | BoardLocation::CommunityChest2
            | BoardLocation::Jail
            | BoardLocation::FreeParking
            | BoardLocation::GoToJail => Self {
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

#[allow(dead_code)]
pub struct BoardSquare {
    position_id: BoardPosition,
    square: BoardLocation,
    cost: usize,
    charge: usize,
    house_cost: usize,
    hotel_cost: usize,
    state: RefCell<PropertyState>,
}

impl BoardSquare {
    pub fn new(
        location: BoardLocation,
        location_config: &HashMap<BoardLocation, Property>,
    ) -> Self {
        let details = location_config.get(&location).unwrap();

        Self {
            position_id: 1,
            state: RefCell::new(PropertyState::new(location)),
            cost: details.tile_cost,
            square: location,
            charge: details.charge,
            house_cost: details.house_cost,
            hotel_cost: details.hotel_cost,
        }
    }

    pub fn is_ownable(&self) -> bool {
        let s = self.state.borrow();
        s.ownable
    }

    pub fn is_owned(&self) -> bool {
        let s = self.state.borrow();
        s.owner.is_some()
    }

    pub fn owner_id(&self) -> Option<usize> {
        let s = self.state.borrow();
        s.owner
    }

    pub fn is_owned_by_player(&self, player: &Player) -> bool {
        match self.state.borrow().owner {
            Some(id) => player.id == id,
            None => false,
        }
    }

    pub const fn get_purchase_cost(&self) -> usize {
        self.cost
    }

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

    pub fn purchase_property(&self, player: &Player) {
        let mut s = self.state.borrow_mut();
        s.owner = Some(player.id);
        player.pay(self.cost);
    }

    pub fn upgradable(&self) -> bool {
        let s = self.state.borrow();
        s.hotel_count < 2 && s.house_count <= 4
    }

    pub fn upgrade_cost(&self) -> usize {
        let s = self.state.borrow();
        if s.house_count < 4 {
            self.house_cost
        } else if s.hotel_count == 0 {
            self.hotel_cost
        } else {
            0
        }
    }

    fn upgrade(&self, player: &Player) {
        let mut s = self.state.borrow_mut();
        if s.house_count <= 4 {
            s.house_count += 1;
            player.add_house();
        } else if s.hotel_count == 0 {
            s.hotel_count += 1;
            player.add_hotel();
        }
    }

    #[allow(dead_code)]
    pub fn purchase_upgrade(&self, player: &Player) {
        player.pay(self.upgrade_cost());

        if player.is_active() {
            self.upgrade(player);
        }
    }

    /// Community Chest
    fn community_chest_space(
        &self,
        player: &Player,
        rng: &mut ThreadRng,
    ) -> (Option<MoveTo>, Option<Payment>, FreeParking) {
        let distribution = Uniform::new_inclusive(1, 14);
        match distribution.sample(rng) {
            1 => {
                log::info!("Community Chest: Advance to Go [Player={}]", player.id);
                (Some(0), None, 0)
            }
            2 => {
                log::info!(
                    "Community Chest: Banking error collect 200 [Player={}]",
                    player.id
                );
                player.deposit(200);
                (None, None, 0)
            }
            3 => {
                log::info!(
                    "Community Chest: Doctors Fees pay 50 [Player={}]",
                    player.id
                );
                player.pay(50);
                (None, None, 50)
            }
            4 => {
                log::info!(
                    "Community Chest: Sale from Stock collect 50 [Player={}]",
                    player.id
                );
                player.deposit(50);
                (None, None, 0)
            }
            5 => {
                log::info!(
                    "Community Chest: Get out of jail free card [Player={}]",
                    player.id
                );
                player.jail_card();
                (None, None, 0)
            }
            6 => {
                log::info!(
                    "Community Chest: Holiday Xmas func matures collect 100 [Player={}]",
                    player.id
                );
                player.deposit(100);
                (None, None, 0)
            }
            7 => {
                log::info!(
                    "Community Chest: Income Tax refund collect 20 [Player={}]",
                    player.id
                );
                player.deposit(20);
                (None, None, 0)
            }
            // 8
            // TODO Its your birthday collect 10 from each player
            9 => {
                log::info!(
                    "Community Chest: Life Insurance matures collect 100 [Player={}]",
                    player.id
                );
                player.deposit(100);
                (None, None, 0)
            }
            10 => {
                log::info!(
                    "Community Chest: Hospital fees pay 50 [Player={}]",
                    player.id
                );
                player.pay(50);
                (None, None, 50)
            }
            11 => {
                log::info!(
                    "Community Chest: Street repairs 40 per house and 115 per hotel [Player={}]",
                    player.id
                );
                let (houses, hotels) = player.count_properties();
                let amount = 40 * houses + 115 * hotels;
                player.pay(amount);
                (None, None, amount)
            }
            12 => {
                log::info!(
                    "Community Chest: Second play in a Beauty competition collect 20 [Player={}]",
                    player.id
                );
                player.deposit(20);
                (None, None, 0)
            }
            _ => {
                log::info!(
                    "Community Chest: Your inheritance comes through collect 100 [Player={}]",
                    player.id
                );
                player.deposit(100);
                (None, None, 0)
            }
        }
    }

    // Chance
    fn chance_space(
        &self,
        player: &Player,
        rng: &mut ThreadRng,
    ) -> (Option<MoveTo>, Option<Payment>, FreeParking) {
        let distribution = Uniform::new_inclusive(1, 16);
        match distribution.sample(rng) {
            1 => {
                log::info!("Chance: Advance to Go [Player={}]", player.id);
                (Some(0), None, 0)
            }
            2 => {
                log::info!("Chance: Go to jail [Player={}]", player.id);
                player.go_to_jail();
                (Some(9), None, 0)
            }
            3 => {
                log::info!(
                    "Chance: Advance to Pall Mall. If you pass Go collection 200 [Player={}]",
                    player.id
                );
                (Some(10), None, 0)
            }
            4 => {
                log::info!("Chance: Take a trip to Marylebone Station and if you pass Go collect 200 [Player={}]", player.id);
                (Some(4), None, 0)
            }
            5 => {
                log::info!(
                    "Chance: Advance to Trafalgar Square. If you pass Go collect 200 [Player={}]",
                    player.id
                );
                (Some(23), None, 0)
            }
            6 => {
                log::info!("Chance: Advance to Mayfair [Player={}]", player.id);
                (Some(38), None, 0)
            }
            7 => {
                log::info!("Chance: Make general repairs on all of your houses. For each house pay \u{a3}25. For each hotel pay \u{a3}100 [Player={}]", player.id);
                let (houses, hotels) = player.count_properties();
                let amount = 25 * houses + 100 * hotels;
                player.pay(amount);
                (None, None, amount)
            }
            8 => {
                log::info!(
                    "Chance: Street repairs 40 per house and 115 per hotel [Player={}]",
                    player.id
                );
                let (houses, hotels) = player.count_properties();
                let amount = 40 * houses + 115 * hotels;
                player.pay(amount);
                (None, None, amount)
            }
            9 => {
                log::info!("Chance: Pay school fees of 150 [Player={}]", player.id);
                player.pay(150);
                (None, None, 150)
            }
            10 => {
                log::info!("Chance: Drunk in charge fine 20 [Player={}]", player.id);
                player.pay(20);
                (None, None, 20)
            }
            11 => {
                log::info!("Chance: Speeding fine pay 16 [Player={}]", player.id);
                player.pay(15);
                (None, None, 15)
            }
            13 => {
                log::info!(
                    "Chance: Your building loan matures. Receive 150 [Player={}]",
                    player.id
                );
                player.deposit(150);
                (None, None, 0)
            }
            14 => {
                log::info!(
                    "Chance: You have won a crossword competition. Collect 100 [Player={}]",
                    player.id
                );
                player.deposit(100);
                (None, None, 0)
            }
            15 => {
                log::info!("Chance: Get out of jail free card [Player={}]", player.id);
                player.jail_card();
                (None, None, 0)
            }
            _ => {
                log::info!(
                    "Chance: Bank pays you dividend of 50 [Player={}]",
                    player.id
                );
                player.deposit(50);
                (Some(0), None, 0)
            }
        }
    }

    #[allow(dead_code)]
    pub fn take_step(
        &self,
        new_position: &Self,
        player: &Player,
        rng: &mut ThreadRng,
    ) -> (Option<MoveTo>, Option<Payment>, FreeParking) {
        match new_position.square {
            BoardLocation::Go => {
                player.deposit(200);
                (None, None, 0)
            }
            BoardLocation::IncomeTax => {
                player.pay(200); // OR 10% of the value of the Assets
                (None, None, 0)
            }
            BoardLocation::LuxuryTax => {
                player.pay(75);
                (None, None, 0)
            }
            BoardLocation::Chance1 | BoardLocation::Chance2 | BoardLocation::Chance3 => {
                self.chance_space(player, rng)
            }
            BoardLocation::CommunityChest1 | BoardLocation::CommunityChest2 => {
                self.community_chest_space(player, rng)
            }
            BoardLocation::GoToJail => {
                player.go_to_jail();
                (Some(9), None, 0)
            }
            BoardLocation::Jail => (None, None, 0), // Just Visiting
            _ => {
                // Not bought
                if self.is_ownable() && player.can_afford(self.get_purchase_cost()) {
                    self.purchase_property(player);
                    return (None, None, 0);
                }

                // Owned by Player - maybe an upgrade?
                let rent = self.rent_cost();
                if self.is_owned_by_player(player) && player.can_afford(self.upgrade_cost()) {
                    self.purchase_upgrade(player);
                    (None, None, 0)
                } else if player.can_afford(rent) {
                    player.pay(rent);

                    (
                        None,
                        Some(Payment {
                            to: self.owner_id().unwrap(),
                            amount: rent,
                            terminal: false,
                        }),
                        0,
                    )
                } else {
                    (
                        None,
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

    use crate::{config::Property, player::Player};

    use super::{BoardLocation, BoardSquare};

    #[test]
    fn test_property_is_ownable() {
        let mut config = HashMap::new();
        config.insert(
            BoardLocation::Piccadilly,
            Property::new(BoardLocation::VineStreet, 300, 50, 100, 200),
        );

        let sq = BoardSquare::new(BoardLocation::Piccadilly, &config);
        assert!(sq.is_ownable());
    }

    #[test]
    fn test_property_is_not_ownable() {
        let mut config = HashMap::new();
        config.insert(
            BoardLocation::Go,
            Property::new(BoardLocation::VineStreet, 0, 0, 0, 0),
        );

        let sq = BoardSquare::new(BoardLocation::Go, &config);
        assert!(!sq.is_ownable());
    }

    #[test]
    fn test_get_cost() {
        let mut config = HashMap::new();
        config.insert(
            BoardLocation::Piccadilly,
            Property::new(BoardLocation::VineStreet, 300, 50, 100, 200),
        );

        let sq = BoardSquare::new(BoardLocation::Piccadilly, &config);
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
        assert_eq!(sq.upgrade_cost(), 200);
        sq.purchase_upgrade(&player_one);
        assert_eq!(sq.rent_cost(), 300);
    }
}
