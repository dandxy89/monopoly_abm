use std::ops::Add;

use rand::distributions::{Distribution, Uniform};
use rand::prelude::ThreadRng;

pub struct RollState {
    pub value: usize,
    pub is_double: bool,
}

impl Default for RollState {
    fn default() -> Self {
        RollState {
            value: 0,
            is_double: true,
        }
    }
}

impl Add for RollState {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            value: self.value + other.value,
            is_double: self.value.eq(&other.value),
        }
    }
}

pub fn roll_game_dice(rng: &mut ThreadRng) -> RollState {
    let distribution = Uniform::new_inclusive(1, 6);
    (0..2).fold(RollState::default(), |a, r| {
        let new = RollState {
            value: distribution.sample(rng),
            is_double: true,
        };

        if r == 0 {
            new
        } else {
            new.add(a)
        }
    })
}

#[cfg(test)]
mod test {
    use super::roll_game_dice;

    #[test]
    fn test_check_rolling_works() {
        let mut r = rand::thread_rng();

        for _ in 0..=1000 {
            let the_roll = roll_game_dice(&mut r);
            assert!(the_roll.value <= 12 && the_roll.value > 1);
        }
    }
}
