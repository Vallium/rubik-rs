#![feature(inclusive_range_syntax)]
#![feature(box_syntax)]
#![feature(test)]

extern crate test;
extern crate bincode;
extern crate serde;

mod move_;
mod cube;
mod cubie;
mod coordinate;
mod solver;

use std::env;
use move_::UserMove;
use cube::Cube;
use solver::Solver;

fn main() {
    let first_arg = env::args().nth(1);
    match first_arg {
        None => {
            println!("Usage")
        }
        Some(arg) => {
            if let Ok(shuffle_sequence) = UserMove::sequence_from_str(&arg) {
                // let cubie = Cubie::from_shuffle_sequence(shuffle_sequence);
                // let mut coordinate = Coordinate::from_cubie(&cubie);
                // coordinate.init_pruning();
                let cube = Cube::from_shuffle_sequence(shuffle_sequence);
                let mut solver = Solver::new(cube);

                solver.solve();
            }
        }
    }
}
