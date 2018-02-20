use cube::Cube;

pub struct Solver {
    max_depth: u8,
    cube: Cube,
}

impl Solver {
    pub fn new(cube: Cube) -> Self {
        Self {
            max_depth: 26,
            cube: cube,
        }
    }

    pub fn solve(&self) {
        self.cube.print();
        println!("Is solved? {}", self.cube.is_solved());
    }
}
