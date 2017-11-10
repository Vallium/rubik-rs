use cube::face::Face;
use cube::CubeMove;

#[derive(Clone, Copy, Hash, Eq, PartialEq)]
pub enum Corner {
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

    pub fn get_face(&self, cubicle: Corner, orientation: u8, face: Face) -> Face {
        let (oriented_a, oriented_b, oriented_c) = (*self).orient(orientation);
        let (a, b, _c) = cubicle.decompose();

        match face {
            x if x == a => oriented_a,
            x if x == b => oriented_b,
            _ => oriented_c,
        }
    }
}

impl ToString for Corner {
    fn to_string(&self) -> String {
        use self::Corner::*;

        match *self {
            URF => "URF",
            UFL => "UFL",
            ULB => "ULB",
            UBR => "UBR",
            DFR => "DFR",
            DLF => "DLF",
            DBL => "DBL",
            DRB => "DRB",
        }.to_string()
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

#[derive(Eq, PartialEq)]
pub struct Corners {
    pub permutations: [Corner; 8],
    pub orientations: [u8; 8],
}

impl Corners {
    pub fn new() -> Self {
        Corners::default()
    }

    fn permute(&mut self, corners: (self::Corner, self::Corner, self::Corner, self::Corner)) {
        let tmp = self.permutations[usize::from(corners.3)];

        self.permutations[usize::from(corners.3)] = self.permutations[usize::from(corners.2)];
        self.permutations[usize::from(corners.2)] = self.permutations[usize::from(corners.1)];
        self.permutations[usize::from(corners.1)] = self.permutations[usize::from(corners.0)];
        self.permutations[usize::from(corners.0)] = tmp;
    }

    fn orient(&mut self, corners: (self::Corner, self::Corner, self::Corner, self::Corner), orients: (u8, u8, u8, u8)) {
        let tmp = self.orientations[usize::from(corners.3)];

        self.orientations[usize::from(corners.3)] = self.orientations[usize::from(corners.2)];
        self.orientations[usize::from(corners.2)] = self.orientations[usize::from(corners.1)];
        self.orientations[usize::from(corners.1)] = self.orientations[usize::from(corners.0)];
        self.orientations[usize::from(corners.0)] = tmp;

        self.orientations[usize::from(corners.3)] = (self.orientations[usize::from(corners.3)] + orients.3) % 3;
        self.orientations[usize::from(corners.2)] = (self.orientations[usize::from(corners.2)] + orients.2) % 3;
        self.orientations[usize::from(corners.1)] = (self.orientations[usize::from(corners.1)] + orients.1) % 3;
        self.orientations[usize::from(corners.0)] = (self.orientations[usize::from(corners.0)] + orients.0) % 3;
    }

    pub fn apply_move(&mut self, m: CubeMove) {
        use self::Corner::*;

        let (corners, orientations) = match m {
            CubeMove::Front => ((URF, DFR, DLF, UFL), (2, 1, 2, 1)),
            CubeMove::FrontPrime => ((URF, UFL, DLF, DFR), (2, 1, 2, 1)),
            CubeMove::Right => ((UBR, DRB, DFR, URF), (2, 1, 2, 1)),
            CubeMove::RightPrime => ((UBR, URF, DFR, DRB), (2, 1, 2, 1)),
            CubeMove::Up => ((URF, UFL, ULB, UBR), (0, 0, 0, 0)),
            CubeMove::UpPrime => ((URF, UBR, ULB, UFL), (0, 0, 0, 0)),
            CubeMove::Back => ((ULB, DBL, DRB, UBR), (2, 1, 2, 1)),
            CubeMove::BackPrime => ((ULB, UBR, DRB, DBL), (2, 1, 2, 1)),
            CubeMove::Left => ((UFL, DLF, DBL, ULB), (2, 1, 2, 1)),
            CubeMove::LeftPrime => ((UFL, ULB, DBL, DLF), (2, 1, 2, 1)),
            CubeMove::Down => ((DRB, DBL, DLF, DFR), (0, 0, 0, 0)),
            CubeMove::DownPrime => ((DRB, DFR, DLF, DBL), (0, 0, 0, 0)),
        };

        self.permute(corners);
        self.orient(corners, orientations);
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
