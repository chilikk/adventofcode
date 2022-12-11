mod aoc;
use std::collections::VecDeque;
use std::iter::FromIterator;
use std::num::ParseIntError;
use std::borrow::Borrow;
use std::env;

struct Monkey {
    items: VecDeque<u64>,
    op: Box<dyn Fn(u64) -> u64>,
    divtest: u64,
    iftrue: usize,
    iffalse: usize,
    ctr: u64,
}

enum Mode {
    Subtask1,
    Subtask2,
}

struct Task11 {
    monkeys: Vec<Monkey>,
    commondiv: u64,
    mode: Mode,
}

impl Task11 {
    fn new(mode: Mode) -> Task11 {
        Task11 {
            monkeys: vec![],
            commondiv: 1,
            mode: mode,
        }
    }

    fn exec_turn(&mut self) {
        let n = self.monkeys.len();
        for i in 0..n {
            let (before, after0) = self.monkeys.split_at_mut(i);
            let (monkey0, after) = after0.split_at_mut(1);
            let monkey = &mut monkey0[0];
            while let Some(item) = monkey.items.pop_front() {
                monkey.ctr += 1;
                let fun: &dyn Fn(u64) -> u64 = monkey.op.borrow();
                let worry = match self.mode {
                    Mode::Subtask1 => fun(item) / 3,
                    Mode::Subtask2 => fun(item) % self.commondiv
                };
                let nextmonkey = if worry % monkey.divtest == 0 {
                    monkey.iftrue
                } else {
                    monkey.iffalse
                };
                if nextmonkey < i {
                    before[nextmonkey].items.push_back(worry)
                } else {
                    after[nextmonkey - i - 1].items.push_back(worry)
                }
            }
        }
    }

    fn res(&self) -> u64 {
        let mut ctrs: Vec<u64> = self.monkeys.iter().map(|m|m.ctr).collect();
        ctrs.sort_by(|a,b|b.cmp(a));
        ctrs[0] * ctrs[1]
    }
}

impl std::fmt::Display for Task11 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.res())
    }
}

impl aoc::AdventurerOfCode for Task11 {
    fn handle_line(&mut self, line: String) {
        if line.starts_with("Monkey ") {
            let i: usize = line.replace("Monkey ", "").replace(":", "").parse().unwrap();
            assert_eq!(i, self.monkeys.len());
            self.monkeys.push(Monkey{
                items: VecDeque::new(),
                op: Box::new(|_| 0),
                divtest: 0,
                iftrue: 0,
                iffalse: 0,
                ctr: 0,
            })
        } else if line.starts_with("  Starting items: ") {
            let effective_line = line.replace("  Starting items: ", "");
            let items = effective_line.split(", ");
            self.monkeys.last_mut().unwrap().items = VecDeque::from_iter(
                items.map(|s|s.parse().unwrap()))
        } else if line.starts_with("  Operation: new = ") {
            let effective_line = line.replace("  Operation: new = ", "");
            let op: Vec<&str> = effective_line
                .split(" ")
                .collect();
            let op1: Result<u64, ParseIntError> = op[0].parse();
            let operator = op[1];
            let op2: Result<u64, ParseIntError> = op[2].parse();
            let opfn: Box<dyn Fn(u64) -> u64> = match (op1, operator, op2) {
                (Ok(v1), "+", Ok(v2)) => Box::new(move |_| v1 + v2),
                (Err(_), "+", Ok(v2)) => Box::new(move |var| var + v2),
                (Ok(v1), "+", Err(_)) => Box::new(move |var| v1 + var),
                (Err(_), "+", Err(_)) => Box::new(move |var| var + var),
                (Ok(v1), "*", Ok(v2)) => Box::new(move |_| v1 * v2),
                (Err(_), "*", Ok(v2)) => Box::new(move |var| var * v2),
                (Ok(v1), "*", Err(_)) => Box::new(move |var| v1 * var),
                (Err(_), "*", Err(_)) => Box::new(move |var| var * var),
                (_, &_, _) => panic!("could not parse"),
            };
            self.monkeys.last_mut().unwrap().op = opfn
        } else if line.starts_with("  Test: divisible by ") {
            let divtest: u64 = line
                .replace("  Test: divisible by ", "")
                .parse()
                .unwrap();
            self.monkeys.last_mut().unwrap().divtest = divtest
        } else if line.starts_with("    If true: throw to monkey ") {
            let m: usize = line
                .replace("    If true: throw to monkey ", "")
                .parse()
                .unwrap();
            assert!(m != self.monkeys.len()-1);
            self.monkeys.last_mut().unwrap().iftrue = m
        } else if line.starts_with("    If false: throw to monkey ") {
            let m: usize = line
                .replace("    If false: throw to monkey ", "")
                .parse()
                .unwrap();
            assert!(m != self.monkeys.len()-1);
            self.monkeys.last_mut().unwrap().iffalse = m
        }
    }

    fn after(&mut self) {
        for div in self.monkeys.iter().map(|m|m.divtest) {
            if self.commondiv % div != 0 {
                self.commondiv *= div;
            }
        };
        let range = match self.mode {
            Mode::Subtask1 => 0..20,
            Mode::Subtask2 => 0..10000,
        };
        for _ in range {
            self.exec_turn();
        }
    }
}

fn main() {
    let args = env::args().collect::<Vec<String>>();
    if args.len() < 2 {
        println!("indicate submode as arg, e.g. `./11 1`");
        return
    }
    let mode = match args[1].as_ref() {
        "1" => Mode::Subtask1,
        "2" => Mode::Subtask2,
        &_ => {
            println!("bad arg, must be `1` or `2`");
            return
        }
    };
    aoc::run(&mut Task11::new(mode))
}
