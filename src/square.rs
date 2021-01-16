use std::{cell::RefCell, collections::HashMap};

#[allow(dead_code)]
#[derive(Debug, PartialEq, Hash)]
pub enum BoardLocation {
    Go,
    OldKentRoad,
    WhitechapelRoad,
    IncomeTax,
    MaryleboneStation,
    TheAngelIslington,
    Chance,
    EustonRoad,
    PentonvilleRoad,
    Jail,
    VisitingJail,
    PallMall,
    ElectricCompany,
    Whitehall,
    NorthumberlandAvenue,
    FenchurchStreetStation,
    BowStreet,
    CommunityChest,
    GreatMarlboroughStreet,
    VineStreet,
    FreeParking,
    TheStrand,
    FleetStreet,
    TrafalgarSquare,
    KingCrossStation,
    LeicesterSquare,
    CoventryStreet,
    WaterCompany,
    Piccadilly,
    GoToJail,
    RegentStreet,
    OxfordStreet,
    BondStreet,
    LiverpoolStreetStation,
    ParkLane,
    Mayfair,
}

impl Eq for BoardLocation {}

#[allow(dead_code)]
pub struct PropertyState {
    pub ownable: bool,
    pub owner: Option<usize>,
    pub house_count: usize,
    pub hotel_count: usize,
    pub mortgaged: bool,
}

impl PropertyState {
    #[allow(dead_code)]
    pub fn new(square: &BoardLocation) -> Self {
        match square {
            BoardLocation::Go
            | BoardLocation::IncomeTax
            | BoardLocation::Chance
            | BoardLocation::Jail
            | BoardLocation::VisitingJail
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
    square: BoardLocation,
    cost: usize,
    charge: usize,
    house_cost: usize,
    hotel_cost: usize,
    state: RefCell<PropertyState>,
}

impl BoardSquare {
    #[allow(dead_code)]
    pub fn new(
        location: BoardLocation,
        location_config: &HashMap<BoardLocation, (usize, usize, usize, usize)>,
    ) -> Self {
        let (tile_cost, charge, house_cost, hotel_cost) = location_config.get(&location).unwrap();

        Self {
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
        if s.hotel_count + s.house_count == 0 {
            self.charge
        } else if s.house_count > 0 {
            s.house_count * self.house_cost
        } else {
            s.hotel_count * self.hotel_cost
        }
    }

    #[allow(dead_code)]
    pub fn purchase(&self, id: usize) {
        let mut s = self.state.borrow_mut();
        s.owner = Some(id);
    }

    #[allow(dead_code)]
    pub fn upgradable(&self) -> bool {
        let s = self.state.borrow();
        s.hotel_count < 2 || s.house_count <= 4
    }

    #[allow(dead_code)]
    pub fn upgrade_cost(&self) -> usize {
        let s = self.state.borrow();
        if s.house_count <= 4 {
            self.house_cost
        } else if s.hotel_count < 2 {
            self.hotel_cost
        } else {
            0
        }
    }
}
