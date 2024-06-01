extern crate rand;
use rand::Rng;

pub struct Agent1;

pub enum Action {
    Up,
    Stay,
}

impl Agent1 {
    pub fn new() -> Self {
        Agent1
    }

    pub fn choose_action(&self) -> Action {
        let mut rng = rand::thread_rng();
        let action = rng.gen_range(0..2);
        match action {
            0 => Action::Up,
            _ => Action::Stay,
        }
    }
}

