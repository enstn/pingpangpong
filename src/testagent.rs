extern crate rand;
use rand::Rng;

pub struct TestAgent;

pub enum Action {
    Up,
    Down,
    Stay,
}

impl TestAgent {
    pub fn new() -> Self {
        TestAgent
    }

    pub fn choose_action(&self) -> Action {
        let mut rng = rand::thread_rng();
        let action = rng.gen_range(0..3);
        match action {
            0 => Action::Up,
            1 => Action::Down,
            _ => Action::Stay,
        }
    }
}

