use cubie::face::Face;
use move_::Move;
use move_::Move_;

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
            1 => (c, a, b),
            _ => (b, c, a),
        }
    }

    pub fn face(&self, cubicle: Corner, orientation: u8, face: Face) -> Face {
        let (oriented_a, oriented_b, oriented_c) = (*self).orient(orientation);
        let (a, b, _c) = cubicle.decompose();

        match face {
            x if x == a => oriented_a,
            x if x == b => oriented_b,
            _ => oriented_c,
        }
    }

    /// Clockwise = false => rotate left
    pub fn rotate_corners_slice(slice: &mut[Corner], begin: usize, end: usize, clockwise: bool) {
        let tmp: Corner;

        if !clockwise {
             tmp = slice[begin];
             for x in begin..end {
                slice[x] = slice[x + 1];
            }
            slice[end] = tmp;
        } else {
            tmp = slice[end];
            for x in (begin + 1..=end).rev() {
                slice[x] = slice[x - 1];
            }
            slice[begin] = tmp;
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

    pub fn multiply(&mut self, m: Move) {
        let m = Move_::move_definition(m);
        let mut new_corners = Self::new();

        for corner in usize::from(Corner::URF)..usize::from(Corner::DRB) + 1 {
            let index = usize::from(m.corners_permutation[corner]);
            new_corners.permutations[corner] = self.permutations[index];

            let ori_a: i8  = self.orientations[index] as i8;
            let ori_b: i8  = m.corners_orientation[corner] as i8;
            let mut ori: i8 = 0;

            if ori_a < 3 && ori_b < 3 {
                ori = ori_a + ori_b;
                if ori >= 3 { ori -= 3; }
            } else if ori_a < 3 && ori_b >= 3 {
                ori = ori_a + ori_b;
                if ori >= 6 { ori -= 3; }
            } else if ori_a >= 3 && ori_b < 3 {
                ori = ori_a - ori_b;
                if ori < 3 { ori += 3; }
            } else if ori_a >= 3 && ori_b >= 3 {
                ori = ori_a - ori_b;
                if ori < 0 { ori += 3; }
            }
            new_corners.orientations[corner] = ori as u8;
        }
        *self = new_corners;
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
