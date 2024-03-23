use crate::{generator::Generator, patient::Patient};

pub struct Simulation {
    generator: Generator,
    pub patients: Vec<Patient>,
}

impl Simulation {
    pub fn new() -> Self {
        let mut patients = Vec::new();
        let mut generator = Generator::new();
        for _ in 0..5 {
            patients.push(generator.random_patient())
        }
        Self { patients, generator }
    }
}
