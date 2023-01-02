#[allow(unused_macros)]
#[macro_use]
mod aoc;

#[derive(Copy,Clone,Debug)]
struct Blizzard {
    init: i16,
    sign: i16, // +1 or -1
    span: i16,
}

impl Blizzard {
    fn negative(pos: usize) -> Self {
        Blizzard{
            init: pos as i16 - 1,
            sign: -1,
            span: 0,
        }
    }
    fn positive(pos: usize) -> Self {
        Blizzard{
            init: pos as i16 - 1,
            sign: 1,
            span: 0,
        }
    }
    fn stationary(pos: usize) -> Self {
        Blizzard{
            init: pos as i16 - 1,
            sign: 0,
            span: 0,
        }
    }
    fn update_span(&mut self, span: usize) {
        self.span = span as i16
    }
    fn position(&self, t: i16) -> usize {
        if self.sign == 0 {
            (self.init + 1) as usize
        } else {
            match (self.init+self.sign*t)%self.span {
                n if n < 0 => (n + 1 + self.span) as usize,
                n           => (n + 1) as usize,
            }
        }
    }
}

struct Task24 {
    start: (usize, usize),
    end: (usize, usize),
    size: (usize, usize),
    vertical: Vec<Vec<Blizzard>>,
    horizontal: Vec<Vec<Blizzard>>,
    time_period: usize,
    temporal_map: Vec<Vec<Vec<bool>>>,
    acc1: usize,
    acc2: usize,
}

impl Task24 {
    fn new() -> Task24 {
        Task24 {
            start: (0,0),
            end: (0,0),
            size: (0,0),
            vertical: Vec::new(),
            horizontal: Vec::new(),
            time_period: 0,
            temporal_map: Vec::new(),
            acc1: 0,
            acc2: 0,
        }
    }

    fn search(&self, stage: u8) -> usize {
        let mut positions = vec![(self.start, stage)];
        let mut t = 0;
        while ! positions.contains(&(self.end, 0)) {
            t += 1;
            let tmap = &self.temporal_map[t % self.time_period];
            let mut new_positions = Vec::new();
            for pos in positions.iter() {
                new_positions.extend(self.next_steps(*pos, tmap));
            }
            positions = new_positions;
            positions.sort();
            positions.dedup();
        }
        t
    }
    fn next_steps(&self, pos: ((usize, usize), u8), tmap: &Vec<Vec<bool>>) -> Vec<((usize,usize),u8)> {
        let mut next_candidates = Vec::new();
        let ((row, col), stage) = pos;
        next_candidates.push(pos);
        if row == 1 && stage % 2 == 0 {
            next_candidates.push(((row-1, col), stage-1))
        } else if row > 0 {
            next_candidates.push(((row-1, col), stage))
        }
        if row == self.horizontal.len() - 2 && stage % 2 == 1 {
            next_candidates.push(((row+1, col), stage-1))
        } else if row < self.horizontal.len() - 1 {
            next_candidates.push(((row+1, col), stage));
        }
        next_candidates.push(((row, col-1), stage));
        next_candidates.push(((row, col+1), stage));
        next_candidates.into_iter().filter(|((r,c), _)| tmap[*r][*c]).collect()
    }

}

impl aoc::AdventurerOfCode for Task24 {
    fn handle_line(&mut self, line: String) {
        self.size.0 += 1;
        if self.size.1 == 0 {
            self.size.1 = line.len();
            self.start = (0, line.chars().position(|c| c == '.').unwrap());
            self.vertical.resize(self.size.1, Vec::new());
        }
        self.horizontal.push(Vec::with_capacity(self.size.1));
        let row = self.size.0-1;
        for (i, c) in line.chars().enumerate() {
            let col = i;
            match c {
                '<' => self.horizontal[row].push(Blizzard::negative(col)),
                '>' => self.horizontal[row].push(Blizzard::positive(col)),
                '^' => self.vertical[col].push(Blizzard::negative(row)),
                'v' => self.vertical[col].push(Blizzard::positive(row)),
                '#' => self.horizontal[row].push(Blizzard::stationary(col)),
                '.' => self.end = (row, col),
                _   => (),
            }
        }
    }

    fn after(&mut self) {
        let (hsize, vsize) = (self.vertical.len()-2, self.horizontal.len()-2);
        self.time_period = {
            // a dumb method to find LCM
            let mut i = 0;
            loop {
                i += 1;
                if i % hsize == 0 && i % vsize == 0 {
                    break i
                }
            }
        };
        self.horizontal.iter_mut().for_each(|bs|bs.iter_mut().for_each(|b|b.update_span(hsize)));
        self.vertical.iter_mut().for_each(|bs|bs.iter_mut().for_each(|b|b.update_span(vsize)));

        self.temporal_map = vec![
            vec![
                vec![true;self.vertical.len()];self.horizontal.len()
            ];self.time_period];
        for t in 0..self.time_period {
            for (row, bs) in self.horizontal.iter().enumerate() {
                for &b in bs.iter() {
                    self.temporal_map[t][row][b.position(t as i16)] = false
                }
            }
            for (col, bs) in self.vertical.iter().enumerate() {
                for &b in bs.iter() {
                    self.temporal_map[t][b.position(t as i16)][col] = false
                }
            }
        }

        self.acc1 = self.search(1);
        self.acc2 = self.search(3);
    }
}

aocfmt!{Task24, self, self.acc1, self.acc2}
aocmain!{Task24}
