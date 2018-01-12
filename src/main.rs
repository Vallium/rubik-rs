mod move_;
mod cube;
mod coordinate;
mod solver;

use std::env;
// use move_::Move;
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
            // if let Ok(shuffle_sequence) = Move::sequence_from_str(&arg) {
                // let cube = Cube::from_shuffle_sequence(shuffle_sequence);
                let cube = Cube::new_default();
                // cube.multiply();
                let coordinate = Coordinate::from_cube(&cube);
                let solver = Solver::new(cube);

                solver.solve();
            // }
        }
    }
}
