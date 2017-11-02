use std::env;

fn parse(arg: String) {
    let splitted = arg.split_whitespace();

    for elem in splitted {
        match elem {
            "F" => println!("F"),
            "F'" => println!("F'"),
            "R" => println!("R"),
            "R'" => println!("R'"),
            "U" => println!("U"),
            "U'" => println!("U'"),
            "B" => println!("B"),
            "B'" => println!("B'"),
            "L" => println!("L"),
            "L'" => println!("L'"),
            "D" => println!("D"),
            "D'" => println!("D'"),
            _ => panic!("Bad arg"),
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
