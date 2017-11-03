use self::Move::*;

#[derive(Clone, Copy)]
pub enum Move {
    Front,
    FrontPrime,
    Front2,
    Right,
    RightPrime,
    Right2,
    Up,
    UpPrime,
    Up2,
    Back,
    BackPrime,
    Back2,
    Left,
    LeftPrime,
    Lefdt2,
    Down,
    DownPrime,
    Down2,
}

impl Move {
    pub fn sequence_from_str(s: &str) -> Result<Vec<Self>, ()> {
        let mut sequence = Vec::new();
        let splitted = s.split_whitespace();

        for elem in splitted {
            let move_ =  match elem {
                "F" => Front,
                "F'" => FrontPrime,
                "F2" => Front2,
                "R" => Right,
                "R'" => RightPrime,
                "R2" => Right2,
                "U" => Up,
                "U'" => UpPrime,
                "U2" => Up2,
                "B" => Back,
                "B'" => BackPrime,
                "B2" => Back2,
                "L" => Left,
                "L'" => LeftPrime,
                "L2" => Lefdt2,
                "D" => Down,
                "D'" => DownPrime,
                "D2" => Down2,
                _ => return Err(()),
            };
            sequence.push(move_)
        }
        Ok(sequence)
    }
}

impl ToString for Move {
    fn to_string(&self) -> String {
        let string = match *self {
            Front => "F",
            FrontPrime => "F'",
            Front2 => "F2",
            Right => "R",
            RightPrime => "R'",
            Right2 => "R2",
            Up => "U",
            UpPrime => "U'",
            Up2 => "U2",
            Back => "B",
            BackPrime => "B'",
            Back2 => "B2",
            Left => "L",
            LeftPrime => "L'",
            Lefdt2 => "L2",
            Down => "D",
            DownPrime => "D'",
            Down2 => "D2",
        };
        string.to_string()
    }
}