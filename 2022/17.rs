#[allow(unused_macros)]
#[macro_use]
mod aoc;
use std::hash::{Hash,Hasher};
use std::collections::hash_map::DefaultHasher;

use Pixel::*;
#[derive(PartialEq,Eq,Copy,Clone,Hash)]
enum Pixel {
    Empty,
    AtRest,
    InMotion,
    Wall,
}
const CHAMBER_WIDTH: usize = 7;
type Row = [Pixel;CHAMBER_WIDTH];
type Chamber = Vec<Row>;
const SHAPE_ORDER: [&'static [Row]; 5] = [
    &[
        [Empty, Empty, Empty, Empty, Empty, Empty, Empty],
        [Empty, Empty, Empty, Empty, Empty, Empty, Empty],
        [Empty, Empty, Empty, Empty, Empty, Empty, Empty],
        [Empty, Empty, InMotion, InMotion, InMotion, InMotion, Empty]
    ],
    &[
        [Empty, Empty, Empty, Empty, Empty, Empty, Empty],
        [Empty, Empty, Empty, Empty, Empty, Empty, Empty],
        [Empty, Empty, Empty, Empty, Empty, Empty, Empty],
        [Empty, Empty, Empty, InMotion, Empty, Empty, Empty],
        [Empty, Empty, InMotion, InMotion, InMotion, Empty, Empty],
        [Empty, Empty, Empty, InMotion, Empty, Empty, Empty]
    ],
    &[
        [Empty, Empty, Empty, Empty, Empty, Empty, Empty],
        [Empty, Empty, Empty, Empty, Empty, Empty, Empty],
        [Empty, Empty, Empty, Empty, Empty, Empty, Empty],
        [Empty, Empty, InMotion, InMotion, InMotion, Empty, Empty],
        [Empty, Empty, Empty, Empty, InMotion, Empty, Empty],
        [Empty, Empty, Empty, Empty, InMotion, Empty, Empty]
    ],
    &[
        [Empty, Empty, Empty, Empty, Empty, Empty, Empty],
        [Empty, Empty, Empty, Empty, Empty, Empty, Empty],
        [Empty, Empty, Empty, Empty, Empty, Empty, Empty],
        [Empty, Empty, InMotion, Empty, Empty, Empty, Empty],
        [Empty, Empty, InMotion, Empty, Empty, Empty, Empty],
        [Empty, Empty, InMotion, Empty, Empty, Empty, Empty],
        [Empty, Empty, InMotion, Empty, Empty, Empty, Empty]
    ],
    &[
        [Empty, Empty, Empty, Empty, Empty, Empty, Empty],
        [Empty, Empty, Empty, Empty, Empty, Empty, Empty],
        [Empty, Empty, Empty, Empty, Empty, Empty, Empty],
        [Empty, Empty, InMotion, InMotion, Empty, Empty, Empty],
        [Empty, Empty, InMotion, InMotion, Empty, Empty, Empty],
    ],
];

#[derive(Copy,Clone)]
enum Direction {
    Left,
    Right,
}

struct StatsState {
    prev_round: usize,
    prev_chamber_len: usize,
    prev_lines_hash: Option<u64>,
    verify_lines: usize,
    extrapolate_chamber_len: Option<usize>,
    extrapolate_finish_round: usize,
}

impl StatsState {
    fn new() -> StatsState {
        StatsState {
            prev_round: 0,
            prev_chamber_len: 0,
            prev_lines_hash: None,
            verify_lines: 10000,
            extrapolate_chamber_len: None,
            extrapolate_finish_round: 0,
        }
    }

    fn hash_last_lines(chamber: &Chamber, lines: usize) -> u64 {
        let mut h = DefaultHasher::new();
        chamber[chamber.len()-lines..chamber.len()].hash(&mut h);
        h.finish()
    }

    fn update_or_skip(&mut self, round: usize, chamber: &Chamber) {
        if let None = self.prev_lines_hash {
            let hash = StatsState::hash_last_lines(chamber, self.verify_lines);
            self.prev_lines_hash = Some(hash);
            self.prev_round = round;
            self.prev_chamber_len = chamber.len()
        }
    }

    fn verify_hash(&self, chamber: &Chamber) -> bool {
        match self.prev_lines_hash {
            Some(hash) =>
                hash == StatsState::hash_last_lines(chamber, self.verify_lines),
            None =>
                false,
        }
    }

    fn save_extrapolate(&mut self, round: usize, rounds: usize, chamber: &Chamber) {
        let diff_rounds = round - self.prev_round;
        let diff_chamber_len = chamber.len() - self.prev_chamber_len;
        let remaining_rounds = rounds - round;
        let multiplier = remaining_rounds / diff_rounds;
        self.extrapolate_chamber_len = Some(diff_chamber_len * multiplier);
        self.extrapolate_finish_round = round + remaining_rounds % diff_rounds;
    }

    fn extrapolate(&self, round: usize, chamber: &Chamber) -> Option<usize> {
        if let Some(extrapolate_chamber_len) = self.extrapolate_chamber_len {
            if self.extrapolate_finish_round == round {
                return Some(chamber.len() + extrapolate_chamber_len)
            }
        }
        None
    }
}

struct Task17 {
    gas_streams: Vec<Direction>,
    acc1: usize,
    acc2: usize,
}

impl Task17 {
    fn new() -> Task17 {
        Task17 {
            gas_streams: Vec::new(),
            acc1: 0,
            acc2: 0,
        }
    }

    fn init_gas_streams(&self, gas_streams: &mut Vec<Direction>) {
        for &dir in self.gas_streams.iter().rev() {
            gas_streams.push(dir);
        }
    }

    fn has_in_motion(r: &Row) -> bool {
        r.iter().filter(|p|**p == InMotion).next() != None
    }

    fn all_empty(r: &Row) -> bool {
        r.iter().filter(|p|**p != Empty).next() == None
    }

    // also includes the rows above the moving shape
    fn get_in_motion_slice_size(chamber: &Chamber) -> usize {
        let end = chamber.len();
        let mut start = end;
        while ! Task17::has_in_motion(&chamber[start-1]) {
            start -= 1;
        }
        while start > 0 && Task17::has_in_motion(&chamber[start-1]) {
            start -= 1;
        }
        end - start
    }

    fn get_in_motion_slice_mut(chamber: &mut Chamber) -> &mut[Row] {
        let size = Task17::get_in_motion_slice_size(chamber);
        let chamber_len = chamber.len();
        &mut chamber[chamber_len-size..chamber_len]
    }

    fn move_sideways(dir: Direction, chamber: &mut Chamber) {
        let slice = Task17::get_in_motion_slice_mut(chamber);
        let mut can_move = true;
        for row in slice.iter() {
            let iter: Box<dyn Iterator<Item=&Pixel>> = match dir {
                Direction::Left => Box::new(row.iter()),
                Direction::Right => Box::new(row.iter().rev()),
            };
            let mut prev = &Wall;
            for p in iter {
                if *p == InMotion && *prev == Empty {
                    break;
                } else if *p == InMotion {
                    can_move = false;
                    break;
                } else {
                    prev = p;
                }
            }
        }
        if can_move {
            for row in slice.iter_mut() {
                let iter: Box<dyn Iterator<Item=&mut Pixel>> = match dir {
                    Direction::Left => Box::new(row.iter_mut()),
                    Direction::Right => Box::new(row.iter_mut().rev()),
                };
                let mut prev = &mut Wall;
                for p in iter {
                    if *p == InMotion {
                        std::mem::swap(p, prev);
                    }
                    prev = p;
                }
            }
        }
    }

    fn move_down(chamber: &mut Chamber) -> bool {
        let chamber_len = chamber.len();
        let slice_len = Task17::get_in_motion_slice_size(chamber);
        let first_in_motion_row_index = chamber.len() - slice_len;
        let mut do_move = true;
        if first_in_motion_row_index == 0 {
            do_move = false;
        } else if Task17::all_empty(&chamber[first_in_motion_row_index - 1]) {
            chamber.remove(first_in_motion_row_index - 1);
        } else {
            let mut prev_row = &chamber[first_in_motion_row_index-1];
            'out: for row in chamber[first_in_motion_row_index..chamber_len].iter() {
                for (i, p) in row.iter().enumerate() {
                    if *p == InMotion && prev_row[i] == AtRest {
                        do_move = false;
                        break 'out;
                    }
                }
                prev_row = row
            }
            if do_move {
                let (chamber_bottom, chamber_top) =
                    chamber.split_at_mut(first_in_motion_row_index);
                let mut prev_row = &mut chamber_bottom[first_in_motion_row_index - 1];
                for row in chamber_top[0..slice_len].iter_mut() {
                    for (i, p) in row.iter_mut().enumerate() {
                        if *p == InMotion {
                            std::mem::swap(p, &mut prev_row[i])
                        }
                    }
                    prev_row = row
                }
                if Task17::all_empty(&chamber[chamber_len - 1]) {
                    chamber.remove(chamber_len - 1);
                }
            }
        }
        if !do_move {
            for row in chamber[first_in_motion_row_index..chamber_len].iter_mut() {
                for p in row.iter_mut() {
                    if *p == InMotion {
                        *p = AtRest;
                    }
                }
            }
        }
        do_move
    }

    fn get_chamber_len_after_rounds(&self, rounds: usize) -> usize {
        let nshapes = SHAPE_ORDER.len();
        let mut chamber: Chamber = Vec::new();
        let mut gas_streams: Vec<Direction> = Vec::new();
        let mut round: usize = 0;
        let mut st = StatsState::new();
        'out: loop {
            let shape = &SHAPE_ORDER[round % nshapes];
            chamber.extend_from_slice(shape);
            loop {
                if gas_streams.len() == 0 {
                    // skip some extra lines due to start irregularities
                    if chamber.len() > 3 * st.verify_lines {
                        if st.verify_hash(&chamber) {
                            st.save_extrapolate(round, rounds, &chamber)
                        } else {
                            st.update_or_skip(round, &chamber)
                        }
                    }
                    self.init_gas_streams(&mut gas_streams);
                }
                Task17::move_sideways(gas_streams.pop().unwrap(), &mut chamber);
                if ! Task17::move_down(&mut chamber) {
                    break
                }
            }
            round += 1;
            if round == rounds {
                break 'out chamber.len();
            } else if let Some(len) = st.extrapolate(round, &chamber) {
                break 'out len;
            }
        }
    }
}

impl aoc::AdventurerOfCode for Task17 {
    fn handle_line(&mut self, line: String) {
        for c in line.chars() {
            match c {
                '<' => self.gas_streams.push(Direction::Left),
                '>' => self.gas_streams.push(Direction::Right),
                _   => panic!("unexpected symbol"),
            }
        }
    }

    fn after(&mut self) {
        self.acc1 = self.get_chamber_len_after_rounds(2022);
        self.acc2 = self.get_chamber_len_after_rounds(1000000000000);
    }
}

aocfmt!{Task17, self, self.acc1, self.acc2}
aocmain!{Task17}
