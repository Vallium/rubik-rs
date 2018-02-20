use cubie::Cubie;

pub struct Solver {
    max_depth: u8,
    cubie: Cubie,
}

impl Solver {
    pub fn new(cubie: Cubie) -> Self {
        Self {
            max_depth: 26,
            cubie: cubie,
        }
    }

    pub fn solve(&self) {
        self.cubie.print();
        println!("Is solved? {}", self.cubie.is_solved());
    }
}
