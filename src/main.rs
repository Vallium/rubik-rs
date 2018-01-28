#![feature(inclusive_range_syntax)]
extern crate bincode;

mod move_;
mod cube;
mod coordinate;
mod solver;

use std::env;
use move_::UserMove;
use cube::Cube;
use coordinate::Coordinate;
use solver::Solver;

fn main() {
    let first_arg = env::args().nth(1);
    match first_arg {
        None => {
            println!("Usage")
        }
        Some(arg) => {
            if let Ok(shuffle_sequence) = UserMove::sequence_from_str(&arg) {
                let cube = Cube::from_shuffle_sequence(shuffle_sequence);
                let mut coordinate = Coordinate::from_cube(&cube);
                coordinate.init_pruning();
                let solver = Solver::new(cube);

                solver.solve();
            }
        }
    }
}
