use std::fs;

use crate::patient::Patient;
use rand::{rngs::ThreadRng, seq::SliceRandom, Rng}; // 0.7.2

pub struct Generator {
    names: Vec<String>,
    rng: ThreadRng,
}

impl Generator {
    pub fn new() -> Generator {
        let mut gen = Generator {
            names: Vec::new(),
            rng: rand::thread_rng(),
        };
        gen.load_names();
        gen
    }

    pub fn random_patient(&mut self) -> Patient {
        Patient {
            name: self.get_random_name(),
            state: rand::random(),
            lep: self.rng.gen_range(5..20),
        }
    }

    fn get_random_name(&self) -> String {
        self.names.choose(&mut rand::thread_rng()).unwrap().clone()
    }

    fn load_names(&mut self) {
        let names = fs::read_to_string("./resource/names.txt").unwrap();
        for line in names.lines() {
            self.names.push(line.to_string());
        }
    }
}
