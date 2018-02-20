#![feature(inclusive_range_syntax)]
#![feature(box_syntax)]
#![feature(test)]

extern crate test;
extern crate bincode;
extern crate serde;

mod move_;
mod cubie;
mod coordinate;
mod solver;

use std::env;
use move_::UserMove;
use cubie::Cubie;
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
                let cubie = Cubie::from_shuffle_sequence(shuffle_sequence);
                let mut coordinate = Coordinate::from_cubie(&cubie);
                coordinate.init_pruning();
                let solver = Solver::new(cubie);

                solver.solve();
            }
        }
    }
}
