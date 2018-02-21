use cubie::Cubie;
use coordinate::Coordinate;
use move_::UserMove;

pub struct Cube {
    cubie: Cubie,
    coordinate: Coordinate,
}

impl Cube {
    pub fn from_shuffle_sequence<I>(shuffle_sequence: I) -> Self
        where I: IntoIterator<Item=(UserMove, usize)> {
        let cubie = Cubie::from_shuffle_sequence(shuffle_sequence);
        let mut coordinate = Coordinate::from_cubie();

        coordinate.init_pruning();

        Self {
            cubie,
            coordinate,
        }
    }

    pub fn cubie(&self) -> &Cubie {
        &self.cubie
    }

    pub fn coordinate(&self) -> &Coordinate {
        &self.coordinate
    }

    pub fn print(&self) {
        self.cubie.print();
    }

    pub fn is_solved(&self) -> bool {
        self.cubie.is_solved()
    }
}
