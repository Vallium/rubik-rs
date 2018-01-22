mod face;
pub mod corners;
pub mod edges;

use cube::face::Face;
use cube::corners::Corners;
use cube::corners::Corner;
use cube::edges::Edges;
use cube::edges::Edge;
use move_::Move;
use move_::UserMove;

#[derive(Eq, PartialEq)]
pub struct Cube {
    corners: Corners,
    edges: Edges,
}

impl Cube {
    pub fn from_shuffle_sequence<I>(shuffle_sequence: I) -> Self
        where I: IntoIterator<Item=(UserMove, usize)>
    {
        let mut new = Self {
            corners: Corners::new(),
            edges: Edges::new(),
            };

        for m in shuffle_sequence.into_iter() {
            let mprime = m.1;
            for _ in 0..mprime {
                new.apply_move(m.0.to_move());
            }
        }
        new
    }

    pub fn new_default() -> Self {
        Self::default()
    }

    pub fn is_solved(&self) -> bool {
        *self == Self::default()
    }

    pub fn multiply(&mut self, m: Move) {
        self.corners.corners_multiply(m);
        self.edges.edges_multiply(m);
    }

    pub fn twist(&self) -> u32 {
        let mut ret: u32 = 0;

        for x in usize::from(Corner::URF)..usize::from(Corner::DRB) {
            ret = 3 * ret + self.corners.orientations[x] as u32;
         }
         ret
    }

    pub fn flip(&self) -> u32 {
        let mut ret: u32 = 0;

        for x in usize::from(Edge::UR)..usize::from(Edge::BR) {
            ret = 2 * ret + self.edges.orientations[x] as u32;
        }
        ret
    }

    pub fn corner_parity(&self) -> u32 {
        let mut ret: u32 = 0;

        for x in (usize::from(Corner::URF) + 1..=usize::from(Corner::DRB)).rev() {
            for y in (usize::from(Corner::URF)..x).rev() {
                if usize::from(self.corners.permutations[y]) > usize::from(self.corners.permutations[x]) {
                    ret += 1;
                }
            }
        }
        ret % 2
    }

    pub fn fr_to_br(&self) -> u32 {
        let mut a: u32 = 0;
        let mut x: u32 = 0;
        let mut edges: [Edge; 4] = [Edge::UR; 4];

        for i in (usize::from(Edge::UR)..=usize::from(Edge::BR)).rev() {
            if usize::from(Edge::FR) <= usize::from(self.edges.permutations[i]) && usize::from(self.edges.permutations[i]) <= usize::from(Edge::BR) {
                a += cnk(11 - i as i16, (x + 1) as i16) as u32;
                edges[3 - x as usize] = self.edges.permutations[i];
                x += 1;
            }
        }

        let mut b: u32 = 0;
        for i in (1..=3).rev() {
            let mut k: u32 = 0;
            loop {
                if usize::from(edges[i]) == i + 8 { break; }
                Edge::rotate_edges_slice(&mut edges, 0, i, false);
                k += 1;
            }
            b = (i as u32 + 1) * b + k;
        }
        24 * a + b
    }

    pub fn urf_to_dlf(&self) -> u32 {
        let mut a: u32 = 0;
        let mut x: u32 = 0;
        let mut corners: [Corner; 6] = [Corner::DBL; 6];

        for i in usize::from(Corner::URF)..=usize::from(Corner::DRB) {
            if usize::from(self.corners.permutations[i]) <= usize::from(Corner::DLF) {
                a += cnk(i as i16, (x + 1) as i16) as u32;
                corners[x as usize] = self.corners.permutations[i];
                x += 1;
            }
        }

        let mut b: u32 = 0;
        for i in (1..=5).rev() {
            let mut k: u32 = 0;
            loop {
                if usize::from(corners[i]) == i { break; }
                Corner::rotate_corners_slice(&mut corners, 0, i, false);
                k += 1;
            }
            b = (i as u32 + 1) * b + k;
        }
        720 * a + b
    }

    pub fn ur_to_ul(&self) -> u32 {
        let mut a: u32 = 0;
        let mut x: u32 = 0;
        let mut edges: [Edge; 3] = [Edge::UR; 3];

        for i in usize::from(Edge::UR)..=usize::from(Edge::BR) {
            if usize::from(self.edges.permutations[i]) <= usize::from(Edge::UL) {
                a += cnk(i as i16, (x + 1) as i16) as u32;
                edges[x as usize] = self.edges.permutations[i];
                x += 1;
            }
        }

        let mut b: u32 = 0;
        for i in (1..=2).rev() {
            let mut k: u32 = 0;
            loop {
                if usize::from(edges[i]) == i { break; }
                Edge::rotate_edges_slice(&mut edges, 0, i, false);
                k += 1;
            }
            b = (i as u32 + 1) * b + k;
        }
        6 * a + b
    }

    pub fn ub_to_df(&self) -> u32 {
        let mut a: u32 = 0;
        let mut x: u32 = 0;
        let mut edges: [Edge; 3] = [Edge::UR; 3];

        for i in usize::from(Edge::UR)..=usize::from(Edge::BR) {
            if usize::from(Edge::UB) <= usize::from(self.edges.permutations[i]) && usize::from(self.edges.permutations[i]) <= usize::from(Edge::DF) {
                a += cnk(i as i16, (x + 1) as i16) as u32;
                edges[x as usize] = self.edges.permutations[i];
                x += 1;
            }
        }

        let mut b: u32 = 0;
        for i in (1..=2).rev() {
            let mut k: u32 = 0;
            loop {
                if usize::from(edges[i]) == usize::from(Edge::UB) + i { break; }
                Edge::rotate_edges_slice(&mut edges, 0, i, false);
                k += 1;
            }
            b = (i as u32 + 1) * b + k;
        }
        6 * a + b
    }

    pub fn ur_to_df(&self) -> u32 {
        let mut a: u32 = 0;
        let mut x: u32 = 0;
        let mut edges: [Edge; 6] = [Edge::UR; 6];

        for i in usize::from(Edge::UR)..=usize::from(Edge::BR) {
            if usize::from(self.edges.permutations[i]) <= usize::from(Edge::DF) {
                a += cnk(i as i16, (x + 1) as i16) as u32;
                edges[x as usize] = self.edges.permutations[i];
                x += 1;
            }
        }

        let mut b: u32 = 0;
        for i in (1..=5).rev() {
            let mut k: u32 = 0;
            loop {
                if usize::from(edges[i]) == i { break; }
                Edge::rotate_edges_slice(&mut edges, 0, i, false);
                k += 1;
            }
            b = (i as u32 + 1) * b + k;
        }
        720 * a + b
    }

    pub fn apply_move(&mut self, m: Move) {
        use move_::Move::*;
        match m {
            Move::Front => self.multiply(Front),
            Move::Right => self.multiply(Right),
            Move::Up => self.multiply(Up),
            Move::Back => self.multiply(Back),
            Move::Left => self.multiply(Left),
            Move::Down => self.multiply(Down),
        }
    }

    fn face(&self, face: Face) -> [Face; 9] {
        use self::corners::Corner::*;
        let corners = match face {
            Face::F => [UFL, URF, DFR, DLF],
            Face::B => [UBR, ULB, DBL, DRB],
            Face::U => [ULB, UBR, URF, UFL],
            Face::D => [DLF, DFR, DRB, DBL],
            Face::L => [ULB, UFL, DLF, DBL],
            Face::R => [URF, UBR, DRB, DFR],
        };

        let mut corner_faces: [self::Face; 4] = [self::Face::F; 4];

        for (i, c) in (&corners).iter().enumerate() {
            let corner_cubie: corners::Corner = self.corners.permutations[usize::from(*c)];

            corner_faces[i] = corner_cubie.face(*c, self.corners.orientations[usize::from(*c)], face);
        }

        use self::edges::Edge::*;
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
            let edge_cubie: edges::Edge = self.edges.permutations[usize::from(*e)];

            edge_faces[i] = edge_cubie.face(*e, self.edges.orientations[usize::from(*e)], face);
        }

        [corner_faces[0], edge_faces[0], corner_faces[1],
        edge_faces[3], face, edge_faces[1],
        corner_faces[3], edge_faces[2], corner_faces[2]]
    }

    pub fn print(&self) {
        let faces = [
            self.face(self::Face::U),
            self.face(self::Face::L),
            self.face(self::Face::F),
            self.face(self::Face::R),
            self.face(self::Face::B),
            self.face(self::Face::D),
        ];

        print!("\n          ");
        for i in 0..9 {
            print!("{} {} \x1b[0m", faces[0][i].color(), faces[0][i].to_string());
            if i > 0 && (i+1) % 3 == 0 {
                print!("\n          ");
            }
        }
        print!("\r\n");
        for y in 0..3 {
            for &face in &faces {
                if face[4] != self::Face::U && face[4] != self::Face::D {
                    for x in 0..3 {
                        print!("{} {} \x1b[0m", face[x+y*3].color(), face[x+y*3].to_string());
                    }
                    print!(" ");
                }
            }
            println!();
        }
        print!("\n          ");
        for i in 0..9 {
            print!("{} {} \x1b[0m", faces[5][i].color(), faces[5][i].to_string());

            if i > 0 && (i+1) % 3 == 0 {
                print!("\n          ");
            }
        }
        println!();
    }
}

impl Default for Cube {
    fn default() -> Self {
        Self {
            corners: corners::Corners::default(),
            edges: edges::Edges::default(),
        }
    }
}

/// Binomial coefficient [n choose k].
pub fn cnk(n: i16, mut k: i16) -> i16 {
    if n < k {
        return 0;
    }
    if k > n / 2 {
        k = n - k;
    }

    let mut i: i16 = n;
    let mut j: i16 = 1;
    let mut s: i16 = 1;

    while i != n - k {
        s *= i;
        s /= j;
        j += 1;
        i -= 1;
    }
    s
}
