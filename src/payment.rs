use crate::player::PlayerId;
pub struct Payment {
    pub to: PlayerId,
    pub amount: usize,
    pub terminal: bool,
}
