use cube::face::Face;
use cube::CubeMove;

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
    RF,
    LF,
    RB,
    LB,
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
            RF => (Face::R, Face::F),
            LF => (Face::L, Face::F),
            RB => (Face::R, Face::B),
            LB => (Face::L, Face::B),
        }
    }

    fn orient(&self, orientation: u8) -> (Face, Face) {
        let (a, b) = (*self).decompose();

        match orientation {
            0 => (a, b),
            _ => (b, a),
        }
    }

    pub fn get_face(&self, cubicle: Edge, orientation: u8, face: Face) -> Face {
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
            RF => 8,
            LF => 9,
            RB => 10,
            LB => 11,
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

    fn permute(&mut self, edges: (self::Edge, self::Edge, self::Edge, self::Edge)) {
        let tmp = self.permutations[usize::from(edges.3)];

        self.permutations[usize::from(edges.3)] = self.permutations[usize::from(edges.2)];
        self.permutations[usize::from(edges.2)] = self.permutations[usize::from(edges.1)];
        self.permutations[usize::from(edges.1)] = self.permutations[usize::from(edges.0)];
        self.permutations[usize::from(edges.0)] = tmp;
    }

    fn orient(&mut self, edges: (self::Edge, self::Edge, self::Edge, self::Edge), orients: (u8, u8, u8, u8)) {
        let tmp = self.orientations[usize::from(edges.3)];

        self.orientations[usize::from(edges.3)] = self.orientations[usize::from(edges.2)];
        self.orientations[usize::from(edges.2)] = self.orientations[usize::from(edges.1)];
        self.orientations[usize::from(edges.1)] = self.orientations[usize::from(edges.0)];
        self.orientations[usize::from(edges.0)] = tmp;

        self.orientations[usize::from(edges.3)] = (self.orientations[usize::from(edges.3)] + orients.3) % 2;
        self.orientations[usize::from(edges.2)] = (self.orientations[usize::from(edges.2)] + orients.2) % 2;
        self.orientations[usize::from(edges.1)] = (self.orientations[usize::from(edges.1)] + orients.1) % 2;
        self.orientations[usize::from(edges.0)] = (self.orientations[usize::from(edges.0)] + orients.0) % 2;
    }

    pub fn apply_move(&mut self, m: CubeMove) {
        use self::Edge::*;

        let (edges, orientations) = match m {
            CubeMove::Front => ((UF, RF, DF, LF), (0, 0, 0, 0)),
            CubeMove::FrontPrime => ((UF, LF, DF, RF), (0, 0, 0, 0)),
            CubeMove::Right => ((UR, RB, DR, RF), (1, 1, 1, 1)),
            CubeMove::RightPrime => ((UR, RF, DR, RB), (1, 1, 1, 1)),
            CubeMove::Up => ((UB, UR, UF, UL), (0, 0, 0, 0)),
            CubeMove::UpPrime => ((UB, UL, UF, UR), (0, 0, 0, 0)),
            CubeMove::Back => ((UB, LB, DB, RB), (0, 0, 0, 0)),
            CubeMove::BackPrime => ((UB, RB, DB, LB), (0, 0, 0, 0)),
            CubeMove::Left => ((UL, LF, DL, LB), (1, 1, 1, 1)),
            CubeMove::LeftPrime => ((UL, LB, DL, LF), (1, 1, 1, 1)),
            CubeMove::Down => ((DF, DR, DB, DL), (0, 0, 0, 0)),
            CubeMove::DownPrime => ((DF, DL, DB, DR), (0, 0, 0, 0)),
        };

        self.permute(edges);
        self.orient(edges, orientations);
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
        edges[usize::from(RF)] = RF;
        edges[usize::from(LF)] = LF;
        edges[usize::from(RB)] = RB;
        edges[usize::from(LB)] = LB;

        Self {
            permutations: edges,
            orientations: [0; 12],
        }
    }
}
