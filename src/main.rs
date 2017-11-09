mod move_;
mod cube;

use std::env;
use move_::Move;
use cube::Cube;

fn main() {
    let first_arg = env::args().nth(1);
    match first_arg {
        None => {
            println!("Usage")
        }
        Some(arg) => {
            if let Ok(shuffle_sequence) = Move::sequence_from_str(&arg) {
                let mut cube = Cube::from_shuffle_sequence(&shuffle_sequence);
                cube.apply_move(Move::Right);
                cube.print();
                cube.apply_move(Move::RightPrime);
                cube.print();
            }
        }
    }
}
