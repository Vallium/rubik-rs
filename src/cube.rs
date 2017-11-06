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
            Face::F => "\x1b[7;33m", // Orange
            Face::B => "\x1b[7;31m", // Red
            Face::U => "\x1b[7;37m", // White
            Face::D => "\x1b[7;30m", // Black
            Face::L => "\x1b[7;32m", // Green
            Face::R => "\x1b[7;34m", // Blue
        }
    }
}

#[derive(Clone, Copy, Hash, Eq, PartialEq)]
enum Corner {
    UFR,
    UFL,
    UBL,
    UBR,
    DFR,
    DFL,
    DBL,
    DBR,
}

impl Corner {
    fn decompose(&self) -> (Face, Face, Face) {
        use self::Corner::*;
        match *self {
            UFR => (Face::U, Face::F, Face::R),
            UFL => (Face::U, Face::F, Face::L),
            UBL => (Face::U, Face::B, Face::L),
            UBR => (Face::U, Face::B, Face::R),
            DFR => (Face::D, Face::F, Face::R),
            DFL => (Face::D, Face::F, Face::L),
            DBL => (Face::D, Face::B, Face::L),
            DBR => (Face::D, Face::B, Face::R),
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

struct Corners {
    map: HashMap<Corner, Corner>,
    orientation: [u8; 8],
}

impl Corners {
    fn new() -> Self {
        Corners::default()
    }
}

impl Default for Corners {
    fn default() -> Self {
        let mut corners = HashMap::new();
        use self::Corner::*;

        corners.insert(UFR, UFR);
        corners.insert(UFL, UFL);
        corners.insert(UBL, UBL);
        corners.insert(UBR, UBR);
        corners.insert(DFR, DFR);
        corners.insert(DFL, DFL);
        corners.insert(DBL, DBL);
        corners.insert(DBR, DBR);

        Self {
            map: corners,
            orientation: [0; 8],
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

struct Edges {
    map: HashMap<Edge, Edge>,
    orientation: [u8; 12],
}

impl Edges {
    fn new() -> Self {
        Edges::default()
    }
}

impl Default for Edges {
    fn default() -> Self {
        let mut edges = HashMap::new();
        use self::Edge::*;

        edges.insert(UR, UR);
        edges.insert(UF, UF);
        edges.insert(UL, UL);
        edges.insert(UB, UB);
        edges.insert(DR, DR);
        edges.insert(DF, DF);
        edges.insert(DL, DL);
        edges.insert(DB, DB);
        edges.insert(FR, FR);
        edges.insert(FL, FL);
        edges.insert(BR, BR);
        edges.insert(BL, BL);

        Self {
            map: edges,
            orientation: [0; 12],
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
}
