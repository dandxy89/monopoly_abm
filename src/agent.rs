use crate::state::State;

pub trait Agent {
    type SimState: State;

    fn step(&mut self, state: &Self::SimState);
}
