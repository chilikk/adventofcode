mod aoc;
use std::collections::BTreeSet;
use std::vec::Vec;

struct Task3 {
    acc1: u32,
    acc2: u32,
    opaque2: Vec<BTreeSet<char>>

}

impl Task3 {
    fn new() -> Task3 {
        return Task3 {
            acc1: 0,
            acc2: 0,
            opaque2: vec![]
        };
    }

    fn priority(sym: char) -> u8 {
        if 'a' <= sym && 'z' >= sym {
            return sym as u8 - 'a' as u8 + 1
        } else if 'A' <= sym && 'Z' >= sym {
            return sym as u8 -'A' as u8 + 27
        } else {
            return 0
        }
    }
}

impl std::fmt::Display for Task3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\n{}", self.acc1, self.acc2)
    }
}

impl aoc::AdventurerOfCode for Task3 {
    fn handle_line(&mut self, line: String) {
        let halflen = line.len() / 2;
        let mut set1 = BTreeSet::<char>::new();
        let mut set2 = BTreeSet::<char>::new();
        let mut setx = BTreeSet::<char>::new();
        for c in (&line[..halflen]).chars() {
            set1.insert(c);
            setx.insert(c);
        }
        for c in (&line[halflen..]).chars() {
            set2.insert(c);
            setx.insert(c);
        }
        for c in set1.intersection(&set2) {
            self.acc1 += Task3::priority(*c) as u32
        }
        self.opaque2.push(setx);
        self.after()
    }

    fn after(&mut self) {
        if self.opaque2.len() == 3 {
            let mut set = BTreeSet::<char>::new();
            for c in self.opaque2[0].intersection(&self.opaque2[1]) {
                set.insert(*c);
            }
            for c in set.intersection(&self.opaque2[2]) {
                self.acc2 += Task3::priority(*c) as u32
            }
            self.opaque2 = vec![];
        }
    }
}

fn main() {
    aoc::run(&mut Task3::new())
}
