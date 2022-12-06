mod aoc;
use std::fmt;
use std::mem;
use std::collections::VecDeque;

type Stacks = Vec<VecDeque<char>>;

struct Task5 {
    stacks: Stacks,
    stacks2: Stacks,
    completed_stacks: bool,
}

impl Task5 {
    fn new() -> Task5 {
        Task5 {
            stacks: vec![],
            stacks2: vec![],
            completed_stacks: false,
        }
    }

    fn fmt_stacks(s: &Stacks, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for q in s {
            match q.front() {
                Some(c) => match write!(f, "{}", c) {
                    Ok(_) => (),
                    err => return err,
                },
                None => (),
            }
        }
        Ok(())
    }

    fn handle_stack_line(&mut self, line: String) {
        let maxidx: usize = line.len() / 4;
        if self.stacks.len() <= maxidx {
            self.stacks.resize(maxidx+1, VecDeque::new())
        }
        let mut iter = line.chars().enumerate().filter(|(i,_)| i % 4 == 1).peekable();
        let (_, c1) = iter.peek().unwrap();
        if *c1 >= '0' && *c1 <= '9' {
            let (_, last) = iter.last().unwrap();
            assert_eq!(self.stacks.len(), String::from(last).parse().unwrap());
            self.stacks2 = self.stacks.clone();
            self.completed_stacks = true
        } else {
            for (i, c) in iter {
                if c != ' ' {
                    let ix: usize = (i - 1) / 4;
                    self.stacks[ix].push_back(c)
                }
            }
        }
    }
}

impl fmt::Display for Task5 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Task5::fmt_stacks(&self.stacks, f).and_then(
            |_|write!(f, "\n")).and_then(
            |_|Task5::fmt_stacks(&self.stacks2, f))
    }
}

impl aoc::AdventurerOfCode for Task5 {
    fn handle_line(&mut self, line: String) {
        if ! self.completed_stacks {
            self.handle_stack_line(line)
        } else if line.starts_with("move ") {
            let line1 = line.strip_prefix("move ").unwrap();
            let (nstr, line2) = line1.split_once(" from ").unwrap();
            let (srcstr, tgtstr) = line2.split_once(" to ").unwrap();
            let n = nstr.parse::<usize>().unwrap();
            let src = srcstr.parse::<usize>().unwrap() - 1;
            let tgt = tgtstr.parse::<usize>().unwrap() - 1;
            for _ in 0..n {
                let c = self.stacks[src].pop_front().unwrap();
                self.stacks[tgt].push_front(c)
            }
            let mut buf = self.stacks2[src].split_off(n);
            mem::swap(&mut buf, &mut self.stacks2[src]);
            buf.append(&mut self.stacks2[tgt]);
            mem::swap(&mut buf, &mut self.stacks2[tgt]);
        }
    }

    fn after(&mut self) {
    }
}

fn main() {
    aoc::run(&mut Task5::new())
}
