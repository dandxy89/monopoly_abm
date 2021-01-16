#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum Token {
    Iron,
    Terrier,
    Horse,
    TopHat,
    Cannon,
    Wheelbarrow,
    Battleship,
    Thimble,
    Car,
    Boot,
}

impl Token {
    pub fn value_to_enum(value: usize) -> Self {
        match value {
            1 => Self::Iron,
            2 => Self::Terrier,
            3 => Self::Horse,
            4 => Self::TopHat,
            5 => Self::Cannon,
            6 => Self::Wheelbarrow,
            7 => Self::Battleship,
            8 => Self::Thimble,
            9 => Self::Car,
            _ => Self::Boot,
        }
    }
}
