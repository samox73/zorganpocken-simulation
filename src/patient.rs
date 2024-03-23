use std::fmt::Display;

use rand::{distributions::{Distribution, Standard}, Rng};

pub enum State {
    Healthy,
    Sick,
    Dead,
}

impl Distribution<State> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> State {
        // match rng.gen_range(0, 3) { // rand 0.5, 0.6, 0.7
        match rng.gen_range(0..=2) { // rand 0.8
            0 => State::Healthy,
            1 => State::Sick,
            _ => State::Dead,
        }
    }
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            State::Healthy => write!(f, "healthy"),
            State::Sick => write!(f, "sick"),
            State::Dead => write!(f, "dead"),
        }
    }
}

pub struct Patient {
    pub name: String,
    pub state: State,
    pub lep: i32,
}