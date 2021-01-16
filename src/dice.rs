use rand::distributions::{Distribution, Uniform};
use rand::prelude::ThreadRng;

#[allow(dead_code)]
fn roll_dice(rng: &mut ThreadRng, n_dice: usize) -> u8 {
    let distribution: Uniform<u8> = Uniform::new_inclusive(1, 6);
    (0..n_dice).map(|_| distribution.sample(rng)).sum()
}

#[cfg(test)]
mod test {
    use super::roll_dice;

    #[test]
    fn test_check_rolling_works() {
        let mut r = rand::thread_rng();

        for _ in 0..=1000 {
            println!("{}", roll_dice(&mut r, 2));
            assert!(roll_dice(&mut r, 2) <= 12);
        }
    }
}
