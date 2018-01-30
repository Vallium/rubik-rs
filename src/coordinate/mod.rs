use cube::Cube;
use move_::Move;

use std::fs;
use std::fs::File;
use std::io::Write;

use bincode;

const NB_MOVES: usize = 18;
const NB_TWIST: usize = 2187;
const NB_FLIP: usize = 2048;
const NB_FR_TO_BR: usize = 11880;
const NB_URF_TO_DLF: usize = 20160;

pub struct Coordinate {
    cache_folder_name: String,
    twist: u32,
    flip: u32,
    parity: u32,
    fr_to_br: u32,
    urf_to_dlf: u32,
    ur_to_ul: u32,
    ub_to_df: u32,
    ur_to_df: u32,
    twist_move: [[u32; NB_MOVES]; NB_TWIST],
    flip_move: [[u32; NB_MOVES]; NB_FLIP],
    fr_to_br_move: [[u32; NB_MOVES]; NB_FR_TO_BR],
    urf_to_dlf_move: [[u32; NB_MOVES]; NB_URF_TO_DLF],
}

impl Coordinate {
    pub fn from_cube(cube: &Cube) -> Self {
        Self {
            cache_folder_name: String::from("pruning_tables"),
            twist: cube.twist(),
            flip: cube.flip(),
            parity: cube.corner_parity(),
            fr_to_br: cube.fr_to_br(),
            urf_to_dlf: cube.urf_to_dlf(),
            ur_to_ul: cube.ur_to_ul(),
            ub_to_df: cube.ub_to_df(),
            ur_to_df: cube.ur_to_df(),
            twist_move: [[0; NB_MOVES]; NB_TWIST],
            flip_move: [[0; NB_MOVES]; NB_FLIP],
            fr_to_br_move: [[0; NB_MOVES]; NB_FR_TO_BR],
            urf_to_dlf_move: [[0; NB_MOVES]; NB_URF_TO_DLF],
        }
    }

    fn create_cache_dir(&self) {
        match fs::create_dir(&self.cache_folder_name) {
            Ok(_) => println!("cache \"folder pruning_tables\" created"),
            Err(e) => println!("{:?}", e),
        }
    }

    fn dump_to_file(&self, arr: &[&[u32]], name: &str) {
        let mut path = self.cache_folder_name.to_owned();
        path.push_str("/");
        path.push_str(name);

        let file = File::create(&path);
        match file {
            Ok(mut file) => {
                let encoded: Vec<u8> = bincode::serialize(&arr, bincode::Infinite).unwrap();
                match file.write(&encoded[..]) {
                    Ok(_) => { },
                    Err (e) => println!("{:?}", e),
                }
                match file.metadata() {
                    Ok(metadata) => {
                        let mut perm = metadata.permissions();
                        perm.set_readonly(true);
                        fs::set_permissions(&path, perm).unwrap();
                    },
                    Err(e) => println!("{:?}", e),
                }
            },
            Err(e) => println!("{:?}", e),
        };
    }

    pub fn init_pruning(&mut self) {
        self.create_cache_dir();
        self.init_twist_move();
        self.init_flip_move();
        self.init_fr_to_br();
        self.init_urf_to_dlf();
    }

    fn init_twist_move(&mut self) {
        let mut solved = Cube::new_default();

        for x in 0..NB_TWIST {
            solved.set_twist(x as u32);
            for y in 0..6 {
                for z in 0..3 {
                    solved.corners_multiply(Move::from_u(y));
                    self.twist_move[x][3 * y + z] = solved.twist();
                }
                solved.corners_multiply(Move::from_u(y));
            }
        }
        self.dump_to_file(&self.twist_move.iter().map(|x| &x[..]).collect::<Vec<&[u32]>>(), "twist_move");
    }

    fn init_flip_move(&mut self) {
        let mut solved = Cube::new_default();

        for x in 0..NB_FLIP {
            solved.set_flip(x as u32);
            for y in 0..6 {
                for z in 0..3 {
                    solved.edges_multiply(Move::from_u(y));
                    self.flip_move[x][3 * y + z] = solved.flip();
                }
                solved.edges_multiply(Move::from_u(y));
            }
        }
        self.dump_to_file(&self.flip_move.iter().map(|x| &x[..]).collect::<Vec<&[u32]>>(), "flip_move");
    }

    fn init_fr_to_br(&mut self) {
        let mut solved = Cube::new_default();

        for x in 0..NB_FR_TO_BR {
            solved.set_fr_to_br(x as i16);
            for y in 0..6 {
                for z in 0..3 {
                    solved.edges_multiply(Move::from_u(y));
                    self.fr_to_br_move[x][3 * y + z] = solved.fr_to_br();
                }
                solved.edges_multiply(Move::from_u(y));
            }
        }
        self.dump_to_file(&self.fr_to_br_move.iter().map(|x| &x[..]).collect::<Vec<&[u32]>>(), "fr_to_br_move");
    }

    fn init_urf_to_dlf(&mut self) {
        let mut solved = Cube::new_default();

        for x in 0..NB_URF_TO_DLF {
            solved.set_urf_to_dlf(x as i16);
            for y in 0..6 {
                for z in 0..3 {
                    solved.corners_multiply(Move::from_u(y));
                    self.urf_to_dlf_move[x][3 * y + z] = solved.urf_to_dlf();
                }
                solved.corners_multiply(Move::from_u(y));
            }
        }
        self.dump_to_file(&self.urf_to_dlf_move.iter().map(|x| &x[..]).collect::<Vec<&[u32]>>(), "urf_to_dlf_move");
    }
}
