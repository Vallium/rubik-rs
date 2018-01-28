use cube::face::Face;
// use cube::CubeMove;
use move_::Move;
use move_::Move_;

#[derive(Clone, Copy, Hash, Eq, PartialEq)]
pub enum Edge {
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
    BL,
    BR,
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
            BL => (Face::B, Face::L),
            BR => (Face::B, Face::R),
        }
    }

    fn orient(&self, orientation: u8) -> (Face, Face) {
        let (a, b) = (*self).decompose();

        match orientation {
            0 => (a, b),
            _ => (b, a),
        }
    }

    pub fn face(&self, cubicle: Edge, orientation: u8, face: Face) -> Face {
        let (oriented_a, oriented_b) = (*self).orient(orientation);
        let (a, _b) = cubicle.decompose();

        match face {
            x if x == a => oriented_a,
            _ => oriented_b,
        }
    }

    /// Clockwise = false => rotate left
    pub fn rotate_edges_slice(slice: &mut[Edge], begin: usize, end: usize, clockwise: bool) {
        let tmp: Edge;

        if !clockwise {
            tmp = slice[begin];
            for x in begin..end {
                slice[x] = slice[x + 1];
            }
            slice[end] = tmp;
        } else {
            tmp = slice[end];
            for x in (begin..end).rev() {
                slice[x] = slice[x - 1];
            }
            slice[begin] = tmp;
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
            BL => 10,
            BR => 11,
        }
    }
}

#[derive(Eq, PartialEq)]
pub struct Edges {
    pub permutations: [Edge; 12],
    pub orientations: [u8; 12],
}

impl Edges {
    pub fn new() -> Self {
        Edges::default()
    }

    pub fn multiply(&mut self, m: Move) {
        let m = Move_::move_definition(m);
        let mut new_edges = Self::new();

        for edge in usize::from(Edge::UR)..usize::from(Edge::BR) + 1 {
            let index = usize::from(m.edges_permutation[edge]);

            new_edges.permutations[edge] = self.permutations[index];
            new_edges.orientations[edge] = (m.edges_orientation[edge] + self.orientations[index]) % 2;
        }
        *self = new_edges;
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
        edges[usize::from(BL)] = BL;
        edges[usize::from(BR)] = BR;

        Self {
            permutations: edges,
            orientations: [0; 12],
        }
    }
}
