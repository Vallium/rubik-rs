extern crate smallvec;

use std::env;
use smallvec::SmallVec;

use self::Move::*;

pub enum Move {
    Front(),
    FrontPrime(),
    Front2(),
    Right(),
    RightPrime(),
    Right2(),
    Up(),
    UpPrime(),
    Up2(),
    Back(),
    BackPrime(),
    Back2(),
    Left(),
    LeftPrime(),
    Lefdt2(),
    Down(),
    DownPrime(),
    Down2(),
}

impl ToString for Move {
    fn to_string(&self) -> String {
        match *self {
            Front() => "F".to_string(),
            FrontPrime() => "F'".to_string(),
            Front2() => "F2".to_string(),
            Right() => "R".to_string(),
            RightPrime() => "R'".to_string(),
            Right2() => "R2".to_string(),
            Up() => "U".to_string(),
            UpPrime() => "U'".to_string(),
            Up2() => "U2".to_string(),
            Back() => "B".to_string(),
            BackPrime() => "B'".to_string(),
            Back2() => "B2".to_string(),
            Left() => "L".to_string(),
            LeftPrime() => "L'".to_string(),
            Lefdt2() => "L2".to_string(),
            Down() => "D".to_string(),
            DownPrime() => "D'".to_string(),
            Down2() => "D2".to_string(),
        }
    }
}

fn parse(arg: String) {
    let splitted = arg.split_whitespace();

    for elem in splitted {
        // if splitted.nth(0) {
            print!(" ");
        // }
        match elem {
            "F" => print!("{}", Front().to_string()),
            "F'" => print!("{}", FrontPrime().to_string()),
            "F2" => print!("{}", Front2().to_string()),
            "R" => print!("{}", Right().to_string()),
            "R'" => print!("{}", RightPrime().to_string()),
            "R2" => print!("{}", Right2().to_string()),
            "U" => print!("{}", Up().to_string()),
            "U'" => print!("{}", UpPrime().to_string()),
            "U2" => print!("{}", Up2().to_string()),
            "B" => print!("{}", Back().to_string()),
            "B'" => print!("{}", BackPrime().to_string()),
            "B2" => print!("{}", Back2().to_string()),
            "L" => print!("{}", Left().to_string()),
            "L'" => print!("{}", LeftPrime().to_string()),
            "L2" => print!("{}", Lefdt2().to_string()),
            "D" => print!("{}", Down().to_string()),
            "D'" => print!("{}", DownPrime().to_string()),
            "D2" => print!("{}", Down2().to_string()),
            _ => panic!("Move unknown: {}", elem),
        }
    }
}

fn main() {
    let first_arg = env::args().nth(1);
    match first_arg {
        None => {
            println!("Usage")
        }
        Some(arg) => {
            parse(arg)
        }
    }
}
