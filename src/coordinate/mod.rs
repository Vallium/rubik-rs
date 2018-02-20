use cubie::Cubie;
use move_::Move;

use std::fs;
use std::fs::File;
use std::io::Write;
use std::io::Read;

use bincode;
use serde;

const NB_MOVES: usize = 18;
const NB_TWIST: usize = 2187;
const NB_FLIP: usize = 2048;
const NB_FR_TO_BR: usize = 11880;
const NB_URF_TO_DLF: usize = 20160;
const NB_UR_TO_UL: usize = 1320;
const NB_UB_TO_DF: usize = 1320;
const NB_UR_TO_DF: usize = 20160;
const NB_SLICE: usize = 24;
const NB_SLICE_TWIST_FLIP: usize = 495;
const NB_PARITY: usize = 2;

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
    twist_move: Box<[[u32; NB_MOVES]]>,
    flip_move: Box<[[u32; NB_MOVES]]>,
    parity_move: Box<[[i8; NB_MOVES]]>,
    fr_to_br_move: Box<[[u32; NB_MOVES]]>,
    urf_to_dlf_move: Box<[[u32; NB_MOVES]]>,
    ur_to_ul_move: Box<[[u32; NB_MOVES]]>,
    ub_to_df_move: Box<[[u32; NB_MOVES]]>,
    ur_to_df_move: Box<[[u32; NB_MOVES]]>,
    merge_ur_to_ul_and_ub_to_df: Box<[[i16; 336]]>,
    urf_to_dlf_parity_prun: Box<[i8]>,
    ur_to_df_parity_prun: Box<[i8]>,
    twist_prun: Box<[i8]>,
    flip_prun: Box<[i8]>,
}

impl Coordinate {
    pub fn from_cubie(cubie: &Cubie) -> Self {
        Self {
            cache_folder_name: String::from("pruning_tables"),
            twist: cubie.twist(),
            flip: cubie.flip(),
            parity: cubie.corner_parity(),
            fr_to_br: cubie.fr_to_br(),
            urf_to_dlf: cubie.urf_to_dlf(),
            ur_to_ul: cubie.ur_to_ul(),
            ub_to_df: cubie.ub_to_df(),
            ur_to_df: cubie.ur_to_df(),
            twist_move: Box::new([[0; NB_MOVES]; NB_TWIST]),
            flip_move: Box::new([[0; NB_MOVES]; NB_FLIP]),
            parity_move: Box::new([
                [1, 0, 1, 1, 0, 1, 1, 0, 1, 1, 0, 1, 1, 0, 1, 1, 0, 1],
                [0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0]]),
            fr_to_br_move: Box::new([[0; NB_MOVES]; NB_FR_TO_BR]),
            urf_to_dlf_move: Box::new([[0; NB_MOVES]; NB_URF_TO_DLF]),
            ur_to_ul_move: Box::new([[0; NB_MOVES]; NB_UR_TO_UL]),
            ub_to_df_move: Box::new([[0; NB_MOVES]; NB_UB_TO_DF]),
            ur_to_df_move: Box::new([[0; NB_MOVES]; NB_UR_TO_DF]),
            merge_ur_to_ul_and_ub_to_df: Box::new([[0; 336]; 336]),
            urf_to_dlf_parity_prun: box [0; NB_SLICE * NB_URF_TO_DLF * NB_PARITY / 2],
            ur_to_df_parity_prun: box [0; NB_SLICE * NB_UR_TO_DF * NB_PARITY / 2],
            twist_prun: box [0; NB_SLICE_TWIST_FLIP * NB_TWIST / 2 + 1],
            flip_prun: box [0; NB_SLICE_TWIST_FLIP * NB_FLIP / 2 + 1],
        }
    }

    fn create_cache_dir(&self) {
        match fs::create_dir(&self.cache_folder_name) {
            Ok(_) => println!("cache \"folder pruning_tables\" created"),
            Err(e) => println!("{:?}", e),
        }
    }

    fn dump_to_file<T>(&self, arr: T, name: &str)
        where T: serde::ser::Serialize {
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

    fn read_cache_table<T>(&self, name: &str) -> Option<T>
        where for<'a> T: serde::Deserialize<'a> {
        let mut path = self.cache_folder_name.to_owned();
        path.push_str("/");
        path.push_str(name);

        let f = File::open(path);
        match f {
            Ok(mut f) => {
                let mut buffer = Vec::new();
                let _ = f.read_to_end(&mut buffer);
                let decoded: T = bincode::deserialize(&buffer).unwrap();
                return Some(decoded);
            },
            Err(e) => println!("{:?}", e),
        }
        None
    }

    pub fn init_pruning(&mut self) {
        self.create_cache_dir();

        match self.read_cache_table::<Box<[[u32; NB_MOVES]]>>("twist_move") {
            Some(a) => {
                println!("reading twist_move from file");
                assert_eq!(a.len(), NB_TWIST);
                self.twist_move = a;
            },
            None => {
                self.init_twist_move();
                println!("dumping twist_move in cache file");
                self.dump_to_file(&self.twist_move[..], "twist_move");
            },
        }

        match self.read_cache_table::<Box<[[u32; NB_MOVES]]>>("flip_move") {
            Some(a) => {
                println!("reading flip_move from file");
                assert_eq!(a.len(), NB_FLIP);
                self.flip_move = a;
            },
            None => {
                self.init_flip_move();
                println!("dumping flip_move in cache file");
                self.dump_to_file(&self.flip_move[..], "flip_move");
            },
        }

        match self.read_cache_table::<Box<[[u32; NB_MOVES]]>>("fr_to_br_move") {
            Some(a) => {
                println!("reading fr_to_br_move from file");
                assert_eq!(a.len(), NB_FR_TO_BR);
                self.fr_to_br_move = a;
            },
            None => {
                self.init_fr_to_br_move();
                println!("dumping fr_to_br_move in cache file");
                self.dump_to_file(&self.fr_to_br_move[..], "fr_to_br_move");
            },
        }

        match self.read_cache_table::<Box<[[u32; NB_MOVES]]>>("urf_to_dlf_move") {
            Some(a) => {
                println!("reading urf_to_dlf_move from file");
                assert_eq!(a.len(), NB_URF_TO_DLF);
                self.urf_to_dlf_move = a;
            },
            None => {
                self.init_urf_to_dlf_move();
                println!("dumping urf_to_dlf_move in cache file");
                self.dump_to_file(&self.urf_to_dlf_move[..], "urf_to_dlf_move");
            },
        }

        match self.read_cache_table::<Box<[[u32; NB_MOVES]]>>("ur_to_ul_move") {
            Some(a) => {
                println!("reading ur_to_ul_move from file");
                assert_eq!(a.len(), NB_UR_TO_UL);
                self.ur_to_ul_move = a;
            },
            None => {
                self.init_ur_to_ul_move();
                println!("dumping ur_to_ul_move in cache file");
                self.dump_to_file(&self.ur_to_ul_move[..], "ur_to_ul_move");
            },
        }

        match self.read_cache_table::<Box<[[u32; NB_MOVES]]>>("ub_to_df_move") {
            Some(a) => {
                println!("reading ub_to_df_move from file");
                assert_eq!(a.len(), NB_UB_TO_DF);
                self.ub_to_df_move = a;
            },
            None => {
                self.init_ub_to_df_move();
                println!("dumping ub_to_df_move in cache file");
                self.dump_to_file(&self.ub_to_df_move[..], "ub_to_df_move");
            },
        }

        match self.read_cache_table::<Box<[[u32; NB_MOVES]]>>("ur_to_df_move") {
            Some(a) => {
                println!("reading ur_to_df_move from file");
                assert_eq!(a.len(), NB_UR_TO_DF);
                self.ur_to_df_move = a;
            },
            None => {
                self.init_ur_to_df_move();
                println!("dumping ur_to_df_move in cache file");
                self.dump_to_file(&self.ur_to_df_move[..], "ur_to_df_move");
            },
        }

        self.init_merge_ur_to_ul_and_ub_to_df();
        // self.dump_to_file(&self.merge_ur_to_ul_and_ub_to_df.iter().map(|x| &x[..]).collect::<Vec<&[i16]>>(), "merge_ur_to_ul_and_ub_to_df");

        match self.read_cache_table::<Box<[i8]>>("urf_to_dlf_parity_prun") {
            Some(a) => {
                println!("reading urf_to_dlf_parity_prun from file");
                assert_eq!(a.len(), NB_SLICE * NB_URF_TO_DLF * NB_PARITY / 2);
                self.urf_to_dlf_parity_prun = a;
            },
            None => {
                self.init_urf_to_dlf_parity_prun();
                println!("dumping urf_to_dlf_parity_prun in cache file");
                self.dump_to_file(&self.urf_to_dlf_parity_prun[..], "urf_to_dlf_parity_prun");
            },
        }

        match self.read_cache_table::<Box<[i8]>>("ur_to_df_parity_prun") {
            Some(a) => {
                println!("reading ur_to_df_parity_prun from file");
                assert_eq!(a.len(), NB_SLICE * NB_UR_TO_DF * NB_PARITY / 2);
                self.ur_to_df_parity_prun = a;
            },
            None => {
                self.init_ur_to_df_parity_prun();
                println!("dumping ur_to_df_parity_prun in cache file");
                self.dump_to_file(&self.ur_to_df_parity_prun[..], "ur_to_df_parity_prun");
            },
        }

        match self.read_cache_table::<Box<[i8]>>("twist_prun") {
            Some(a) => {
                println!("reading twist_prun from file");
                assert_eq!(a.len(), NB_SLICE_TWIST_FLIP * NB_TWIST / 2 + 1);
                self.twist_prun = a;
            },
            None => {
                self.init_twist_prun();
                println!("dumping twist_prun in cache file");
                self.dump_to_file(&self.twist_prun[..], "twist_prun");
            },
        }

        match self.read_cache_table::<Box<[i8]>>("flip_prun") {
            Some(a) => {
                println!("reading flip_prun from file");
                assert_eq!(a.len(), NB_SLICE_TWIST_FLIP * NB_FLIP / 2 + 1);
                self.flip_prun = a;
            },
            None => {
                self.init_flip_prun();
                println!("dumping flip_prun in cache file");
                self.dump_to_file(&self.flip_prun[..], "flip_prun");
            },
        }
    }

    fn init_twist_move(&mut self) {
        let mut solved = Cubie::new_default();

        for x in 0..NB_TWIST {
            solved.set_twist(x as i16);
            for y in 0..6 {
                for z in 0..3 {
                    solved.corners_multiply(Move::from_u(y));
                    self.twist_move[x][3 * y + z] = solved.twist();
                }
                solved.corners_multiply(Move::from_u(y));
            }
        }
    }

    fn init_flip_move(&mut self) {
        let mut solved = Cubie::new_default();

        for x in 0..NB_FLIP {
            solved.set_flip(x as i16);
            for y in 0..6 {
                for z in 0..3 {
                    solved.edges_multiply(Move::from_u(y));
                    self.flip_move[x][3 * y + z] = solved.flip();
                }
                solved.edges_multiply(Move::from_u(y));
            }
        }
    }

    fn init_fr_to_br_move(&mut self) {
        let mut solved = Cubie::new_default();

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
    }

    fn init_urf_to_dlf_move(&mut self) {
        let mut solved = Cubie::new_default();

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
    }

    fn init_ur_to_ul_move(&mut self) {
        let mut solved = Cubie::new_default();

        for x in 0..NB_UR_TO_UL {
            solved.set_ur_to_ul(x as i16);
            for y in 0..6 {
                for z in 0..3 {
                    solved.edges_multiply(Move::from_u(y));
                    self.ur_to_ul_move[x][3 * y + z] = solved.ur_to_ul();
                }
                solved.edges_multiply(Move::from_u(y));
            }
        }
    }

    fn init_ub_to_df_move(&mut self) {
        let mut solved = Cubie::new_default();

        for x in 0..NB_UR_TO_UL {
            solved.set_ub_to_df(x as i16);
            for y in 0..6 {
                for z in 0..3 {
                    solved.edges_multiply(Move::from_u(y));
                    self.ub_to_df_move[x][3 * y + z] = solved.ub_to_df();
                }
                solved.edges_multiply(Move::from_u(y));
            }
        }
    }

    fn init_ur_to_df_move(&mut self) {
        let mut solved = Cubie::new_default();

        for x in 0..NB_UR_TO_DF {
            solved.set_ur_to_df(x as i16);
            for y in 0..6 {
                for z in 0..3 {
                    solved.edges_multiply(Move::from_u(y));
                    self.ur_to_df_move[x][3 * y + z] = solved.ur_to_df();
                }
                solved.edges_multiply(Move::from_u(y));
            }
        }
    }

    fn init_merge_ur_to_ul_and_ub_to_df(&mut self) {
        for ur_to_ul in 0..336 {
            for ub_to_df in 0..336 {
                self.merge_ur_to_ul_and_ub_to_df[ur_to_ul][ub_to_df] = Cubie::ur_to_uf_standalone(ur_to_ul as i16, ub_to_df as i16);
            }
        }
    }

    fn init_urf_to_dlf_parity_prun(&mut self) {
        self.urf_to_dlf_parity_prun = Box::new([-1; NB_SLICE * NB_URF_TO_DLF * NB_PARITY / 2]);
        let mut depth = 0;
        let mut done = 1;

        Self::set_prunning(&mut self.urf_to_dlf_parity_prun[..], 0, 0);

        loop {
            if done == NB_SLICE * NB_URF_TO_DLF * NB_PARITY { break; }

            for x in 0..NB_SLICE * NB_URF_TO_DLF * NB_PARITY {
                let parity = x % 2;
                let urf_to_dlf = (x / 2) / NB_SLICE;
                let slice = (x / 2) % NB_SLICE;
                if Self::prunning(&self.urf_to_dlf_parity_prun[..], x) == depth {
                    for y in 0..NB_MOVES {
                        match y {
                            3 | 5 | 6 | 8 | 12 | 14 | 15 | 17 => continue,
                            _ => {
                                let n_slice = self.fr_to_br_move[slice][y];
                                let n_urf_to_dlf = self.urf_to_dlf_move[urf_to_dlf][y];
                                let n_parity = self.parity_move[parity][y];
                                let index = ((NB_SLICE as i32 * n_urf_to_dlf as i32 + n_slice as i32) * 2 + n_parity as i32) as usize;
                                if Self::prunning(&self.urf_to_dlf_parity_prun[..], index) == 0x0f {
                                    Self::set_prunning(&mut self.urf_to_dlf_parity_prun[..], index, depth + 1);
                                    done += 1;
                                }
                            },
                        }
                    }
                }
            }
            depth += 1;
        }
    }

    fn init_ur_to_df_parity_prun(&mut self) {
        self.ur_to_df_parity_prun = Box::new([-1; NB_SLICE * NB_UR_TO_DF * NB_PARITY / 2]);
        let mut depth = 0;
        let mut done = 1;

        Self::set_prunning(&mut self.ur_to_df_parity_prun[..], 0, 0);

        loop {
            if done == NB_SLICE * NB_UR_TO_DF * NB_PARITY { break; }

            for x in 0..NB_SLICE * NB_UR_TO_DF * NB_PARITY {
                let parity = x % 2;
                let ur_to_df = (x / 2) / NB_SLICE;
                let slice = (x / 2) % NB_SLICE;
                if Self::prunning(&self.ur_to_df_parity_prun[..], x) == depth {
                    for y in 0..NB_MOVES {
                        match y {
                            3 | 5 | 6 | 8 | 12 | 14 | 15 | 17 => continue,
                            _ => {
                                let n_slice = self.fr_to_br_move[slice][y];
                                let n_ur_to_df = self.ur_to_df_move[ur_to_df][y];
                                let n_parity = self.parity_move[parity][y];
                                let index = ((NB_SLICE as i32 * n_ur_to_df as i32 + n_slice as i32) * 2 + n_parity as i32) as usize;
                                if Self::prunning(&self.ur_to_df_parity_prun[..], index) == 0x0f {
                                    Self::set_prunning(&mut self.ur_to_df_parity_prun[..], index, depth + 1);
                                    done += 1;
                                }
                            },
                        }
                    }
                }
            }
            depth += 1;
        }
    }

    fn init_twist_prun(&mut self) {
        self.twist_prun = box [-1 ; NB_SLICE_TWIST_FLIP * NB_TWIST / 2 + 1];
        let mut depth = 0;
        let mut done = 1;

        Self::set_prunning(&mut self.twist_prun[..], 0, 0);

        loop {
            if done == NB_SLICE_TWIST_FLIP * NB_TWIST { break; }

            for x in 0..NB_SLICE_TWIST_FLIP * NB_TWIST {
                let twist = x / NB_SLICE_TWIST_FLIP;
                let slice = x % NB_SLICE_TWIST_FLIP;
                if Self::prunning(&self.twist_prun[..], x) == depth {
                    for y in 0..NB_MOVES {
                        let n_twist = self.twist_move[twist][y];
                        let n_slice = self.fr_to_br_move[slice * 24][y] / 24;
                        let index = (NB_SLICE_TWIST_FLIP as i32 * n_twist as i32 + n_slice as i32) as usize;
                        if Self::prunning(&self.twist_prun[..], index) == 0x0f {
                            Self::set_prunning(&mut self.twist_prun[..], index, depth + 1);
                            done += 1;
                        }
                    }
                }
            }
            depth += 1;
        }
    }

    fn init_flip_prun(&mut self) {
        self.flip_prun = box [-1 ; NB_SLICE_TWIST_FLIP * NB_FLIP / 2 + 1];
        let mut depth = 0;
        let mut done = 1;

        Self::set_prunning(&mut self.flip_prun[..], 0, 0);

        loop {
            if done == NB_SLICE_TWIST_FLIP * NB_FLIP { break; }

            for x in 0..NB_SLICE_TWIST_FLIP * NB_FLIP {
                let flip = x / NB_SLICE_TWIST_FLIP;
                let slice = x % NB_SLICE_TWIST_FLIP;
                if Self::prunning(&self.flip_prun[..], x) == depth {
                    for y in 0..NB_MOVES {
                        let n_flip = self.flip_move[flip][y];
                        let n_slice = self.fr_to_br_move[slice * 24][y] / 24;
                        let index = (NB_SLICE_TWIST_FLIP as i32 * n_flip as i32 + n_slice as i32) as usize;
                        if Self::prunning(&self.flip_prun[..], index) == 0x0f {
                            Self::set_prunning(&mut self.flip_prun[..], index, depth + 1);
                            done += 1;
                        }
                    }
                }
            }
            depth += 1;
        }
    }

    fn set_prunning(arr: &mut [i8], i: usize, value: i8) {
        if i & 1 == 0 {
            arr[i / 2] &= 0xf0 | value;
        } else {
            arr[i / 2] &= 0x0f | (value << 4);
        }
    }

    fn prunning(arr: &[i8], i: usize) -> i8{
        let ret: i8;

        if i & 1 == 0 {
            ret = arr[i / 2] & 0x0f;
        } else {
            ret = (arr[i / 2] >> 4) & 0x0f;
        }
        ret
    }
}
