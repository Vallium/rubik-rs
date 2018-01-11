
use cube::Cube;

pub struct Coordinate {
    // cube_moves: [u8; 8],
    twist: u32,
    flip: u8,
    parity: u8,
    FRtoBR: u8,
    URFtoDLF: u8,
    URtoUL: u8,
    UBtoDF: u8,
    URtoDF: u16,
}

impl Coordinate {
    pub fn from_cube(cube: &Cube) -> Self {
        Self {
            // cube_moves: [0; 8],
            twist: cube.get_twist(),
            flip: 0,
            parity: 0,
            FRtoBR: 0,
            URFtoDLF: 0,
            URtoUL: 0,
            UBtoDF: 0,
            URtoDF: 0,
        }
    }
}
