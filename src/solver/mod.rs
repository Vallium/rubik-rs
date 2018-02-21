use cube::Cube;
use coordinate::Coordinate;
use std::cmp;

const NB_SLICE_TWIST_FLIP: usize = 495;

pub struct Solver {
    max_depth: u8,
    cube: Cube,
    axis_powers: [(i8, i8); 31],
    flip: [u32; 31],
    twist: [u32; 31],
    slice: [u32; 31],
    parity: [i8; 31],
    urf_to_dlf: [u32; 31],
    fr_to_br: [u32; 31],
    ur_to_ul: [u32; 31],
    ub_to_df: [u32; 31],
    ur_to_df: [u32; 31],
    min_dist_p1: [u32; 31],
    min_dist_p2: [u32; 31],
}

impl Solver {
    pub fn new(cube: Cube) -> Self {
        Self {
            max_depth: 24,
            cube,
            axis_powers: [(0, 0); 31],
            flip: [0; 31],
            twist: [0; 31],
            slice: [0; 31],
            parity: [0; 31],
            urf_to_dlf: [0; 31],
            fr_to_br: [0; 31],
            ur_to_ul: [0; 31],
            ub_to_df: [0; 31],
            ur_to_df: [0; 31],
            min_dist_p1: [0; 31],
            min_dist_p2: [0; 31],
        }
    }

    pub fn solve(&mut self) {
        self.flip[0] = self.cube.cubie().flip();
        self.twist[0] = self.cube.cubie().twist();
        self.slice[0] = self.cube.cubie().fr_to_br() / 24;
        self.parity[0] = self.cube.cubie().corner_parity();
        self.urf_to_dlf[0] = self.cube.cubie().urf_to_dlf();
        self.fr_to_br[0] = self.cube.cubie().fr_to_br();
        self.ur_to_ul[0] = self.cube.cubie().ur_to_ul();
        self.ub_to_df[0] = self.cube.cubie().ub_to_df();
        self.ur_to_df[0] = self.cube.cubie().ur_to_df();
        self.min_dist_p1[1] = 1;

        let mut n = 0;
        let mut busy = false;
        let mut depth_phase_1 = 1;

        loop {
            loop {
                if depth_phase_1 - n > self.min_dist_p1[n + 1] as usize && busy == false {
                    if self.axis_powers[n].0 == 0 || self.axis_powers[n].0 == 3 {
                        n += 1;
                        self.axis_powers[n].0 = 1;
                    } else {
                        n += 1;
                        self.axis_powers[n].0 = 0;
                    }
                    self.axis_powers[n].1 = 1;
                } else if self.axis_powers[n].1 > 3 {
                    self.axis_powers[n].1 += 1;

                    loop {
                        self.axis_powers[n].0 += 1;

                        if self.axis_powers[n].0 > 5 {
                            if n == 0 {
                                if depth_phase_1 >= self.max_depth as usize {
                                    unimplemented!();
                                } else {
                                    depth_phase_1 += 1;
                                    self.axis_powers[n].0 = 0;
                                    self.axis_powers[n].1 = 1;
                                    busy = false;
                                    break;
                                }
                            } else {
                                n -= 1;
                                busy = true;
                                break;
                            }
                        } else {
                            self.axis_powers[n].1 = 1;
                            busy == false;
                        }
                        if n == 0 || (self.axis_powers[n - 1].0 != self.axis_powers[n].0 && self.axis_powers[n - 1].0 - 3 != self.axis_powers[n].0) { break; }
                    }
                } else {
                    self.axis_powers[n].1 += 1;
                    busy = false;
                }
                if busy == false { break; }
            }


            let mv = 3 * self.axis_powers[n].0 + self.axis_powers[n].1 - 1;

            self.twist[n + 1] = self.cube.coordinate().twist_move[n][mv as usize];
            self.flip[n + 1] = self.cube.coordinate().twist_move[n][mv as usize];
            self.slice[n + 1] = self.cube.coordinate().fr_to_br_move[(self.slice[n] * 24) as usize][mv as usize] / 24;

            self.min_dist_p1[n + 1] = cmp::max(
                Coordinate::prunning(&self.cube.coordinate().flip_prun, NB_SLICE_TWIST_FLIP *  self.flip[n + 1] as usize + self.slice[n + 1] as usize) as u32,
                Coordinate::prunning(&self.cube.coordinate().flip_prun, NB_SLICE_TWIST_FLIP *  self.twist[n + 1] as usize + self.slice[n + 1] as usize) as u32
            );

            if self.min_dist_p1[n + 1] == 0 && n >= depth_phase_1 - 5 {
                self.min_dist_p1[n + 1] = 10;
                let s = self.total_depth(depth_phase_1 as u8);
                // if n == depth_phase_1 - 1 {

                // }
            }
        }
        // println!("Is solved? {}", self.cube.is_solved());
    }

    fn total_depth(&mut self, depth_phase_1: u8) -> Option<u8> {
        let mut mv: i8 = 0;
        let max_depth_phase_2 = cmp::min(10, self.max_depth - depth_phase_1);

        for x in 0..depth_phase_1 {
            mv = 3 * self.axis_powers[x as usize].0 + self.axis_powers[x as usize].1 - 1;
            self.urf_to_dlf[x as usize + 1] = self.cube.coordinate().urf_to_dlf_move[self.urf_to_dlf[x as usize] as usize][mv as usize];
            self.fr_to_br[x as usize + 1] = self.cube.coordinate().fr_to_br_move[self.fr_to_br[x as usize] as usize][mv as usize];
            self.parity[x as usize + 1] = self.cube.coordinate().parity_move[self.parity[x as usize] as usize][mv as usize];
        }

        let d1 = Coordinate::prunning(&self.cube.coordinate().urf_to_dlf_parity_prun, ((NB_SLICE_TWIST_FLIP as u32 * self.urf_to_dlf[depth_phase_1 as usize] * self.fr_to_br[depth_phase_1 as usize]) * 2 + self.parity[depth_phase_1 as usize] as u32) as usize);
        if d1 > max_depth_phase_2 as i8 {
            return None;
        }

        self.ur_to_df[depth_phase_1 as usize] = self.cube.coordinate().merge_ur_to_ul_and_ub_to_df[self.ur_to_ul[depth_phase_1 as usize] as usize][self.ub_to_df[depth_phase_1 as usize] as usize] as u32;

        let d2 = Coordinate::prunning(&self.cube.coordinate().ur_to_df_parity_prun, ((NB_SLICE_TWIST_FLIP as u32 * self.ur_to_df[depth_phase_1 as usize] * self.fr_to_br[depth_phase_1 as usize]) * 2 + self.parity[depth_phase_1 as usize] as u32) as usize);
        if d2 > max_depth_phase_2 as i8 {
            return None;
        }

        self.min_dist_p2[depth_phase_1 as usize] = cmp::max(d1 as u32, d2 as u32);
        if self.min_dist_p2[depth_phase_1 as usize] == 0 {
            // Already solved
            return Some(depth_phase_1);
        }

        let depth_phase_2: u8 = 1;
        let mut n: u8 = depth_phase_1;
        let busy: bool = false;

        self.axis_powers[depth_phase_1 as usize] = (0, 0);
        self.min_dist_p2[n as usize + 1] = 1;

        loop {
            loop {
                if depth_phase_1 + depth_phase_2 - n > self.min_dist_p2[n as usize + 1] as u8 && busy == false {
                    if self.axis_powers[n as usize].0 == 0 || self.axis_powers[n as usize].0 == 3 {
                        n += 1;
                        self.axis_powers[n as usize] = (1, 2);
                    } else {
                        n += 1;
                        self.axis_powers[n as usize] = (0, 1);
                    }
                }
                if busy == false { break; }
            }
            if self.min_dist_p2[n as usize + 1] == 0 { break; }
        }
        Some(depth_phase_1 + depth_phase_2)
    }
}
