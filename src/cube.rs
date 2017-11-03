
use move_::Move;

pub struct Cube {
    shuffle_sequence: Vec<Move>,
}

impl Cube {
    pub fn new() -> Self {
        Self {
            shuffle_sequence: Vec::new(),
        }
    }

    pub fn from_shuffle_sequence(shuffle_sequence: &Vec<Move>) -> Self {
        Self {
            shuffle_sequence: (*shuffle_sequence).clone(),
        }
    }
}