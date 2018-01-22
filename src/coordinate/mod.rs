
use cube::Cube;

pub struct Coordinate {
    twist: u32,
    flip: u32,
    parity: u32,
    fr_to_br: u32,
    urf_to_dlf: u32,
    ur_to_ul: u32,
    ub_to_df: u32,
    ur_to_df: u32,
}

impl Coordinate {
    pub fn from_cube(cube: &Cube) -> Self {
        Self {
            twist: cube.twist(),
            flip: cube.flip(),
            parity: cube.corner_parity(),
            fr_to_br: cube.fr_to_br(),
            urf_to_dlf: cube.urf_to_dlf(),
            ur_to_ul: cube.ur_to_ul(),
            ub_to_df: cube.ub_to_df(),
            ur_to_df: cube.ur_to_df(),
        }
    }
}
