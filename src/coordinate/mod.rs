
use cube::Cube;

pub struct Coordinate {
    // cube_moves: [u8; 8],
    twist: u32,
    flip: u8,
    parity: u8,
    fr_to_br: u8,
    urf_to_dlf: u8,
    ur_to_ul: u8,
    ub_to_df: u8,
    ur_to_df: u16,
}

impl Coordinate {
    pub fn from_cube(cube: &Cube) -> Self {
        Self {
            // cube_moves: [0; 8],
            twist: 0,//cube.get_twist(),
            flip: 0,
            parity: 0,
            fr_to_br: 0,
            urf_to_dlf: 0,
            ur_to_ul: 0,
            ub_to_df: 0,
            ur_to_df: 0,
        }
    }
}
