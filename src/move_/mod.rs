use cube::corners::Corner;
use cube::edges::Edge;

pub enum Move {
    Up,
    Right,
    Front,
    Down,
    Left,
    Back,
}

pub struct Move_ {
    pub corners_permutation: [Corner; 8],
    pub corners_orientation: [u8; 8],
    pub edges_permutation: [Edge; 12],
    pub edges_orientation: [u8; 12],
}

impl Move_ {
    pub fn move_definition(m: Move) -> Self {
        use cube::corners::Corner::*;
        use cube::edges::Edge::*;
        use self::Move::*;
        match m {
            Up => Self {
                corners_permutation: [UBR, URF, UFL, ULB, DFR, DLF, DBL, DRB],
                corners_orientation: [0; 8],
                edges_permutation: [UB, UR, UF, UL, DR, DF, DL, DB, FR, FL, BL, BR],
                edges_orientation: [0; 12],
            },
            Right => Self {
                corners_permutation: [DFR, UFL, ULB, URF, DRB, DLF, DBL, UBR],
                corners_orientation: [2, 0, 0, 1, 1, 0, 0, 2],
                edges_permutation: [FR, UF, UL, UB, BR, DF, DL, DB, DR, FL, BL, UR],
                edges_orientation: [0; 12],
            },
            Front => Self {
                corners_permutation: [UFL, DLF, ULB, UBR, URF, DFR, DBL, DRB],
                corners_orientation: [1, 2, 0, 0, 2, 1, 0, 0],
                edges_permutation: [UR, FL, UL, UB, DR, FR, DL, DB, UF, DF, BL, BR],
                edges_orientation: [0, 1, 0, 0, 0, 1, 0, 0, 1, 1, 0, 0],
            },
            Down => Self {
                corners_permutation: [URF, UFL, ULB, UBR, DLF, DBL, DRB, DFR],
                corners_orientation: [0; 8],
                edges_permutation: [UR, UF, UL, UB, DF, DL, DB, DR, FR, FL, BL, BR],
                edges_orientation: [0; 12],
            },
            Left => Self {
                corners_permutation: [URF, ULB, DBL, UBR, DFR, UFL, DLF, DRB],
                corners_orientation: [0, 1, 2, 0, 0, 2, 1, 0],
                edges_permutation: [UR, UF, BL, UB, DR, DF, FL, DB, FR, UL, DL, BR],
                edges_orientation: [0; 12],
            },
            Back => Self {
                corners_permutation: [URF, UFL, UBR, DRB, DFR, DLF, ULB, DBL],
                corners_orientation: [0, 0, 1, 2, 0, 0, 2, 1],
                edges_permutation: [UR, UF, UL, BR, DR, DF, DL, BL, FR, FL, UB, DB],
                edges_orientation: [0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 1, 1],
            },
        }
    }
}


// use self::Move::*;

// #[derive(Clone, Copy)]
// pub enum Move {
//     Front,
//     FrontPrime,
//     Front2,
//     Right,
//     RightPrime,
//     Right2,
//     Up,
//     UpPrime,
//     Up2,
//     Back,
//     BackPrime,
//     Back2,
//     Left,
//     LeftPrime,
//     Left2,
//     Down,
//     DownPrime,
//     Down2,
// }

// impl Move {
//     pub fn sequence_from_str(s: &str) -> Result<Vec<Self>, ()> {
//         let mut sequence = Vec::new();
//         let splitted = s.split_whitespace();

//         for elem in splitted {
//             let move_ =  match elem {
//                 "F" => Front,
//                 "F'" => FrontPrime,
//                 "F2" => Front2,
//                 "R" => Right,
//                 "R'" => RightPrime,
//                 "R2" => Right2,
//                 "U" => Up,
//                 "U'" => UpPrime,
//                 "U2" => Up2,
//                 "B" => Back,
//                 "B'" => BackPrime,
//                 "B2" => Back2,
//                 "L" => Left,
//                 "L'" => LeftPrime,
//                 "L2" => Left2,
//                 "D" => Down,
//                 "D'" => DownPrime,
//                 "D2" => Down2,
//                 _ => return Err(()),
//             };
//             sequence.push(move_)
//         }
//         Ok(sequence)
//     }
// }

// impl ToString for Move {
//     fn to_string(&self) -> String {
//         let string = match *self {
//             Front => "F",
//             FrontPrime => "F'",
//             Front2 => "F2",
//             Right => "R",
//             RightPrime => "R'",
//             Right2 => "R2",
//             Up => "U",
//             UpPrime => "U'",
//             Up2 => "U2",
//             Back => "B",
//             BackPrime => "B'",
//             Back2 => "B2",
//             Left => "L",
//             LeftPrime => "L'",
//             Left2 => "L2",
//             Down => "D",
//             DownPrime => "D'",
//             Down2 => "D2",
//         };
//         string.to_string()
//     }
// }
