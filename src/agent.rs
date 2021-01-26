use crate::state::State;

pub trait Agent {
    type SimState: State;

    fn step(&self, state: &Self::SimState);
}
