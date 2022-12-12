mod aoc;
use std::collections::BTreeSet;
use std::iter::Extend;

struct Task12 {
    map: Vec<i8>,
    l: usize,
    c: usize,
    start: usize,
    end: usize
}

enum Mode {
    Subtask1,
    Subtask2,
}

impl Task12 {
    fn new() -> Task12 {
        Task12 {
            map: vec![],
            l: 0,
            c: 0,
            start: 0,
            end: 0
        }
    }

    fn path(&self, mode: Mode) -> u16 {
        let mut seen: Vec<i16> = vec![-1; self.map.len()];
        let mut next: BTreeSet<usize> = BTreeSet::new();
        next.insert(self.end);
        let mut step: u16 = 0;
        while self.condition(&mode, &next) {
            let mut cur = Vec::new();
            cur.extend(next.iter());
            next = BTreeSet::new();
            for i in cur.iter().map(|x: &usize| *x as i16) {
                for j in [i+1, i-1, i+self.l as i16, i-self.l as i16] {
                    if self.in_boundaries(j) && self.not_seen(&seen, j) && self.can_climb(j, i) {
                        seen[j as usize] = i;
                        next.insert(j as usize);
                    }
                }
            };
            step += 1
        };
        step
    }

    fn condition(&self, mode: &Mode, next: &BTreeSet<usize>) -> bool {
        match *mode {
            Mode::Subtask1 => !next.contains(&self.start),
            Mode::Subtask2 => {
                for i in next.iter() {
                    if self.map[*i] == 0 {
                        return false;
                    }
                };
                true
            },
        }

    }

    fn in_boundaries(&self, i: i16) -> bool {
        i>=0 && i<self.map.len() as i16
    }

    fn not_seen(&self, seen: &Vec<i16>, j: i16) -> bool {
        seen[j as usize] == -1
    }

    fn can_climb(&self, i: i16, j: i16) -> bool {
        self.map[j as usize] <= self.map[i as usize]+1
    }

    fn task1(&self) -> u16 {
        self.path(Mode::Subtask1)
    }

    fn task2(&self) -> u16 {
        self.path(Mode::Subtask2)
    }
}

impl std::fmt::Display for Task12 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\n{}", self.task1(), self.task2())
    }
}

impl aoc::AdventurerOfCode for Task12 {
    fn handle_line(&mut self, line: String) {
        if self.l == 0 {
            self.l = line.len()
        }
        for (i, c) in line.chars().enumerate() {
            let v = match c {
                'S' => {
                    self.start = self.c * self.l + i as usize;
                    0
                },
                'E' => {
                    self.end = self.c * self.l + i as usize;
                    'z' as i8 - 'a' as i8
                },
                _ => c as i8 - 'a' as i8
            };
            self.map.push(v)
        };
        self.c += 1 as usize
    }

    fn after(&mut self) {
    }
}

fn main() {
    aoc::run(&mut Task12::new())
}
