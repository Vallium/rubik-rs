#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Face {
    F, // Front
    B, // Back
    U, // Up
    D, // Down
    L, // Left
    R, // Right
}

impl Face {
    pub fn color(&self) -> &str {
        match *self {
            Face::F => "\x1b[7;33m", // Yellow
            Face::B => "\x1b[7;31m", // Red
            Face::U => "\x1b[7;37m", // White
            Face::D => "\x1b[7;47;30m", // Black
            Face::L => "\x1b[7;32m", // Green
            Face::R => "\x1b[7;34m", // Blue
        }

    }
}

impl ToString for Face {
    fn to_string(&self) -> String {
        match *self {
            Face::F => "F",
            Face::B => "B",
            Face::U => "U",
            Face::D => "D",
            Face::L => "L",
            Face::R => "R",
        }.to_string()
    }
}
