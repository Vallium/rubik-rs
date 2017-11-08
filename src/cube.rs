use std::collections::HashMap;

use move_::Move;

#[derive(Clone, Copy, Eq, PartialEq)]
enum Face {
    F, // Front
    B, // Back
    U, // Up
    D, // Down
    L, // Left
    R, // Right
}

impl Face {
    fn color(&self) -> &str {
        match *self {
            Face::F => "\x1b[7;33m", // Yellow
            Face::B => "\x1b[7;31m", // Red
            Face::U => "\x1b[7;37m", // White
            Face::D => "\x1b[7;47;30m", // Black
            Face::L => "\x1b[7;32m", // Green
            Face::R => "\x1b[7;34m", // Blue
        }

    }
}

impl ToString for Face {
    fn to_string(&self) -> String {
        match *self {
            Face::F => "F",
            Face::B => "B",
            Face::U => "U",
            Face::D => "D",
            Face::L => "L",
            Face::R => "R",
        }.to_string()
    }
}

#[derive(Clone, Copy, Hash, Eq, PartialEq)]
enum Corner {
    URF,
    UFL,
    ULB,
    UBR,
    DFR,
    DLF,
    DBL,
    DRB,
}

impl Corner {
    fn decompose(&self) -> (Face, Face, Face) {
        use self::Corner::*;
        match *self {
            URF => (Face::U, Face::R, Face::F),
            UFL => (Face::U, Face::F, Face::L),
            ULB => (Face::U, Face::L, Face::B),
            UBR => (Face::U, Face::B, Face::R),
            DFR => (Face::D, Face::F, Face::R),
            DLF => (Face::D, Face::L, Face::F),
            DBL => (Face::D, Face::B, Face::L),
            DRB => (Face::D, Face::R, Face::B),
        }
    }

    fn orient(&self, orientation: u8) -> (Face, Face, Face) {
        let (a, b, c) = (*self).decompose();

        match orientation {
            0 => (a, b, c),
            1 => (b, c, a),
            _ => (c, a, b),
        }
    }

    fn get_face(&self, cubicle: Corner, orientation: u8, face: Face) -> Face {
        let (oriented_a, oriented_b, oriented_c) = (*self).orient(orientation);
        let (a, b, _c) = cubicle.decompose();

        match face {
            x if x == a => oriented_a,
            x if x == b => oriented_b,
            _ => oriented_c,
        }
    }
}

impl From<Corner> for usize {
    fn from(c: Corner) -> Self {
        use self::Corner::*;

        match c {
            URF => 0,
            UFL => 1,
            ULB => 2,
            UBR => 3,
            DFR => 4,
            DLF => 5,
            DBL => 6,
            DRB => 7,
        }
    }
}

struct Corners {
    permutations: [Corner; 8],
    orientations: [u8; 8],
}

impl Corners {
    fn new() -> Self {
        Corners::default()
    }

    fn apply_move(&mut self, m: Move) {
        use self::Corner::*;
        let corners = match m {
            Move::Front => (URF, DFR, DLF, UFL),
            Move::FrontPrime => (URF, UFL, DLF, DFR),
            Move::Right => (UBR, DRB, DFR, URF),
            Move::RightPrime => (UBR, URF, DFR, DRB),
            Move::Up => (URF, UFL, ULB, UBR),
            Move::UpPrime => (URF, UBR, ULB, UFL),
            Move::Back => (ULB, DBL, DRB, UBR),
            Move::BackPrime => (ULB, UBR, DRB, DBL),
            Move::Left => (UFL, DLF, DBL, ULB),
            Move::LeftPrime => (UFL, ULB, DBL, DLF),
            Move::Down => (DRB, DBL, DLF, DFR),
            Move::DownPrime => (DRB, DFR, DLF, DBL),
            _ => unimplemented!(),
        };

        let tmp = self.permutations[usize::from(corners.3)];

        self.permutations[usize::from(corners.3)] = self.permutations[usize::from(corners.2)];
        self.permutations[usize::from(corners.2)] = self.permutations[usize::from(corners.1)];
        self.permutations[usize::from(corners.1)] = self.permutations[usize::from(corners.0)];
        self.permutations[usize::from(corners.0)] = tmp;

        let tmp2 = self.orientations[usize::from(corners.3)];

        self.orientations[usize::from(corners.3)] = self.orientations[usize::from(corners.2)];
        self.orientations[usize::from(corners.2)] = self.orientations[usize::from(corners.1)];
        self.orientations[usize::from(corners.1)] = self.orientations[usize::from(corners.0)];
        self.orientations[usize::from(corners.0)] = tmp2;

        self.orientations[usize::from(corners.3)] = (self.orientations[usize::from(corners.3)] + 1) % 3;
        self.orientations[usize::from(corners.2)] = (self.orientations[usize::from(corners.2)] + 2) % 3;
        self.orientations[usize::from(corners.1)] = (self.orientations[usize::from(corners.1)] + 1) % 3;
        self.orientations[usize::from(corners.0)] = (self.orientations[usize::from(corners.0)] + 2) % 3;
    }
}

impl Default for Corners {
    fn default() -> Self {
        let mut corners: [Corner; 8] = [URF; 8];
        use self::Corner::*;

        corners[usize::from(URF)] = URF;
        corners[usize::from(UFL)] = UFL;
        corners[usize::from(ULB)] = ULB;
        corners[usize::from(UBR)] = UBR;
        corners[usize::from(DFR)] = DFR;
        corners[usize::from(DLF)] = DLF;
        corners[usize::from(DBL)] = DBL;
        corners[usize::from(DRB)] = DRB;

        Self {
            permutations: corners,
            orientations: [0; 8],
        }
    }
}

#[derive(Clone, Copy, Hash, Eq, PartialEq)]
enum Edge {
    UR,
    UF,
    UL,
    UB,
    DR,
    DF,
    DL,
    DB,
    FR,
    FL,
    BR,
    BL,
}

impl Edge {
    fn decompose(&self) -> (Face, Face) {
        use self::Edge::*;
        match *self {
            UR => (Face::U, Face::R),
            UF => (Face::U, Face::F),
            UL => (Face::U, Face::L),
            UB => (Face::U, Face::B),
            DR => (Face::D, Face::R),
            DF => (Face::D, Face::F),
            DL => (Face::D, Face::L),
            DB => (Face::D, Face::B),
            FR => (Face::F, Face::R),
            FL => (Face::F, Face::L),
            BR => (Face::B, Face::R),
            BL => (Face::B, Face::L),
        }
    }

    fn orient(&self, orientation: u8) -> (Face, Face) {
        let (a, b) = (*self).decompose();

        match orientation {
            0 => (a, b),
            _ => (b, a),
        }
    }

    fn get_face(&self, cubicle: Edge, orientation: u8, face: Face) -> Face {
        let (oriented_a, oriented_b) = (*self).orient(orientation);
        let (a, _b) = cubicle.decompose();

        match face {
            x if x == a => oriented_a,
            _ => oriented_b,
        }
    }
}

impl From<Edge> for usize {
    fn from(e: Edge) -> Self {
        use self::Edge::*;

        match e {
            UR => 0,
            UF => 1,
            UL => 2,
            UB => 3,
            DR => 4,
            DF => 5,
            DL => 6,
            DB => 7,
            FR => 8,
            FL => 9,
            BR => 10,
            BL => 11,
        }
    }
}

struct Edges {
    permutations: [Edge; 12],
    orientations: [u8; 12],
}

impl Edges {
    fn new() -> Self {
        Edges::default()
    }
}

impl Default for Edges {
    fn default() -> Self {
        let mut edges: [Edge; 12] = [UR; 12];
        use self::Edge::*;

        edges[usize::from(UR)] = UR;
        edges[usize::from(UF)] = UF;
        edges[usize::from(UL)] = UL;
        edges[usize::from(UB)] = UB;
        edges[usize::from(DR)] = DR;
        edges[usize::from(DF)] = DF;
        edges[usize::from(DL)] = DL;
        edges[usize::from(DB)] = DB;
        edges[usize::from(FR)] = FR;
        edges[usize::from(FL)] = FL;
        edges[usize::from(BR)] = BR;
        edges[usize::from(BL)] = BL;

        Self {
            permutations: edges,
            orientations: [0; 12],
        }
    }
}

pub struct Cube {
    shuffle_sequence: Vec<Move>,
    corners: Corners,
    edges: Edges,
}

impl Cube {
    pub fn new() -> Self {
        Self {
            shuffle_sequence: Vec::new(),
            corners: Corners::new(),
            edges: Edges::new(),
        }
    }

    pub fn from_shuffle_sequence(shuffle_sequence: &Vec<Move>) -> Self {
        Self {
            shuffle_sequence: (*shuffle_sequence).clone(),
            corners: Corners::new(),
            edges: Edges::new(),
        }
    }

    fn get_face(&self, face: Face) -> [Face; 9] {
        use self::Corner::*;
        let corners = match face {
            Face::F => [UFL, URF, DFR, DLF],
            Face::B => [ULB, UBR, DBL, DRB],
            Face::U => [ULB, UBR, URF, UFL],
            Face::D => [DLF, DFR, DRB, DBL],
            Face::L => [ULB, UFL, DLF, DBL],
            Face::R => [URF, UBR, DRB, DFR],
        };

        let mut corner_faces: [self::Face; 4] = [self::Face::F; 4];

        for (i, c) in (&corners).iter().enumerate() {
            let corner_cubie: Corner = self.corners.permutations[usize::from(*c)];

            corner_faces[i] = corner_cubie.get_face(*c, self.corners.orientations[usize::from(*c)], face);
        }

        use self::Edge::*;
        let edges = match face {
            Face::F => [UF, FR, DF, FL],
            Face::B => [UB, BL, DB, BR],
            Face::U => [UB, UR, UF, UL],
            Face::D => [DF, DR, DB, DL],
            Face::L => [UL, FL, DL, BL],
            Face::R => [UR, BR, DR, FR],
        };

        let mut edge_faces: [self::Face; 4] = [self::Face::F; 4];

        for (i, e) in (&edges).iter().enumerate() {
            let edge_cubie: Edge = self.edges.permutations[usize::from(*e)];

            edge_faces[i] = edge_cubie.get_face(*e, self.edges.orientations[usize::from(*e)], face);
        }

        [corner_faces[0], edge_faces[0], corner_faces[1],
        edge_faces[3], face, edge_faces[1],
        corner_faces[3], edge_faces[2], corner_faces[2]]
    }

    pub fn print(&self) {
        let faces = [
            self.get_face(self::Face::U),
            self.get_face(self::Face::L),
            self.get_face(self::Face::F),
            self.get_face(self::Face::R),
            self.get_face(self::Face::B),
            self.get_face(self::Face::D),
        ];

        print!("\n         ");
        for i in 0..9 {
            print!("{} {} \x1b[0m", faces[0][i].color(), faces[0][i].to_string());
            if i > 0 && (i+1) % 3 == 0 {
                print!("\n         ");
            }
        }
        print!("\r");
        for y in 0..3 {
            for &face in &faces {
                if face[4] != self::Face::U && face[4] != self::Face::D {
                    for x in 0..3 {
                        print!("{} {} \x1b[0m", face[x+y*3].color(), face[x+y*3].to_string());
                    }
                }
            }
            print!("\n");
        }
        print!("         ");
        for i in 0..9 {
            print!("{} {} \x1b[0m", faces[5][i].color(), faces[5][i].to_string());

            if i > 0 && (i+1) % 3 == 0 {
                print!("\n         ");
            }
        }
        print!("\r");
    }
}
