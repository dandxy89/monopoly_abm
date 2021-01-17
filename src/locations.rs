#[allow(dead_code)]
#[derive(Debug, PartialEq, Hash)]
pub enum BoardLocationName {
    Go,
    OldKentRoad,
    WhitechapelRoad,
    IncomeTax,
    MaryleboneStation,
    TheAngelIslington,
    Chance1,
    EustonRoad,
    PentonvilleRoad,
    Jail,
    PallMall,
    ElectricCompany,
    Whitehall,
    NorthumberlandAvenue,
    FenchurchStreetStation,
    BowStreet,
    CommunityChest1,
    GreatMarlboroughStreet,
    VineStreet,
    FreeParking,
    TheStrand,
    Chance2,
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
    CommunityChest2,
    BondStreet,
    LiverpoolStreetStation,
    Chance3,
    ParkLane,
    LuxuryTax,
    Mayfair,
}

impl Eq for BoardLocationName {}

// impl BoardLocationName {
//     #[allow(dead_code)]
//     fn position(position: usize) -> Self {
//         match position {
//             0 => Self::Go,
//             1 => Self::OldKentRoad,
//             2 => Self::WhitechapelRoad,
//             3 => Self::IncomeTax,
//             4 => Self::MaryleboneStation,
//             5 => Self::TheAngelIslington,
//             6 => Self::Chance1,
//             7 => Self::EustonRoad,
//             8 => Self::PentonvilleRoad,
//             9 => Self::Jail,
//             10 => Self::PallMall,
//             11 => Self::ElectricCompany,
//             12 => Self::Whitehall,
//             13 => Self::NorthumberlandAvenue,
//             14 => Self::FenchurchStreetStation,
//             15 => Self::BowStreet,
//             16 => Self::CommunityChest1,
//             17 => Self::GreatMarlboroughStreet,
//             18 => Self::VineStreet,
//             19 => Self::FreeParking,
//             20 => Self::TheStrand,
//             21 => Self::Chance2,
//             22 => Self::FleetStreet,
//             23 => Self::TrafalgarSquare,
//             24 => Self::KingCrossStation,
//             25 => Self::LeicesterSquare,
//             26 => Self::CoventryStreet,
//             27 => Self::WaterCompany,
//             28 => Self::Piccadilly,
//             29 => Self::GoToJail,
//             30 => Self::RegentStreet,
//             31 => Self::OxfordStreet,
//             32 => Self::CommunityChest2,
//             33 => Self::BondStreet,
//             34 => Self::LiverpoolStreetStation,
//             35 => Self::Chance3,
//             36 => Self::ParkLane,
//             37 => Self::LuxuryTax,
//             _ => Self::Mayfair,
//         }
//     }
// }

// #[cfg(test)]
// mod test {
//     use super::BoardLocationName;

//     #[test]
//     fn get_location() {
//         let current_location = 3;
//         let move_steps = 3;
//         assert_eq!(
//             BoardLocationName::position(current_location),
//             BoardLocationName::IncomeTax
//         );
//         assert_eq!(
//             BoardLocationName::position(current_location + move_steps),
//             BoardLocationName::Chance1
//         );
//     }
// }
