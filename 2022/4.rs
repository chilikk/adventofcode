mod aoc;
use std::fmt;

struct Task4 {
    acc1: u16,
    acc2: u16,
}

impl Task4 {
    fn new() -> Task4 {
        Task4 {
            acc1: 0,
            acc2: 0
        }
    }
}

impl fmt::Display for Task4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}\n{}", self.acc1, self.acc2)
    }
}

impl aoc::AdventurerOfCode for Task4 {
    fn handle_line(&mut self, line: String) {
        let (range1, range2) = line.split_once(",").unwrap();
        let (r1s, r1e) = range1.split_once("-").unwrap();
        let (r2s, r2e) = range2.split_once("-").unwrap();
        let r1si: u32 = r1s.parse().unwrap();
        let r1ei: u32 = r1e.parse().unwrap();
        let r2si: u32 = r2s.parse().unwrap();
        let r2ei: u32 = r2e.parse().unwrap();
        if r1si <= r2si && r1ei >= r2ei || r2si <= r1si && r2ei >= r1ei {
            self.acc1 += 1
        }
        if r1si <= r2ei && r1ei >= r2si {
            self.acc2 += 1
        }
    }

    fn after(&mut self) {
    }
}

fn main() {
    aoc::run(&mut Task4::new())
}
