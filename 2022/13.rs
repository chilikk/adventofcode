mod aoc;
use std::cmp::{PartialOrd, Ordering};

#[derive(Clone)]
enum Elem {
    None,
    Int(i16),
    List(Vec<Elem>),
}

impl PartialEq for Elem {
    fn eq(&self, other: &Elem) -> bool {
        self.cmp(other) == Ordering::Equal
    }

}

impl Eq for Elem {
}

impl PartialOrd for Elem {
    fn partial_cmp(&self, other: &Elem) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Elem {
    fn cmp(&self, other: &Elem) -> Ordering {
        match (self, other) {
            (Elem::Int(a), Elem::Int(b)) =>
                a.cmp(b),
            (Elem::Int(v), Elem::List(_)) =>
                Elem::List(vec![Elem::Int(*v)]).cmp(other),
            (Elem::List(_), Elem::Int(v)) =>
                self.cmp(&Elem::List(vec![Elem::Int(*v)])),
            (Elem::List(l1), Elem::List(l2)) => {
                let len = l1.len().min(l2.len());
                for i in 0..len {
                    match l1[i].cmp(&l2[i]) {
                        Ordering::Equal => (),
                        cmp => return cmp,
                    }
                };
                l1.len().cmp(&l2.len())
            },
            (_, _) =>
                panic!("cannot compare")
        }
    }

}

struct Task13 {
    l1: Elem,
    i: u16,
    acc1: u16,
    all: Vec<Elem>,
    acc2: u16,
    m2: Elem,
    m6: Elem,
}

macro_rules! marker {
    ($x:expr) => { Elem::List(vec![Elem::List(vec![Elem::Int($x)])]) }
}

impl Task13 {
    fn new() -> Task13 {
        let m2 = marker!(2);
        let m6 = marker!(6);
        Task13 {
            l1: Elem::None,
            i: 1,
            acc1: 0,
            all: vec![m2.clone(), m6.clone()],
            acc2: 1,
            m2: m2,
            m6: m6
        }
    }

    fn get_mut(l: &mut Elem, level: u8) -> &mut Elem {
        let mut r = l;
        for _ in 1..level {
            r = match r {
                Elem::List(v) => v,
                &mut _        => panic!("not a list"),
            }.last_mut().unwrap();
        }
        r
    }

    fn task1(&self) -> u16 {
        self.acc1
    }

    fn task2(&self) -> u16 {
        self.acc2
    }
}

impl std::fmt::Display for Task13 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\n{}", self.task1(), self.task2())
    }
}

impl aoc::AdventurerOfCode for Task13 {
    fn handle_line(&mut self, line: String) {
        let mut l = Elem::None;
        let mut level: u8 = 0;
        let mut last = &mut l;
        for c in line.chars() {
            match c {
                '[' => {
                    match last {
                        Elem::List(v) => {
                            v.push(Elem::List(vec![]));
                            level += 1;
                            last = v.last_mut().unwrap();
                        },
                        Elem::None if level == 0 => {
                            *last = Elem::List(vec![]);
                            level = 1
                        },
                        &mut _ => panic!("not a list"),
                    }
                },
                ']' => {
                    match last {
                        Elem::List(_) => level -= 1,
                        Elem::Int(_) => level -= 2,
                        Elem::None => panic!("unexpected character"),
                    }
                    last = Task13::get_mut(&mut l, level)
                },
                ',' =>
                    match last {
                        Elem::List(_) => (),
                        Elem::Int(_) => {
                            level -= 1;
                            last = Task13::get_mut(&mut l, level)
                        },
                        Elem::None => panic!("unexpected character"),
                    },
                '0'..='9' => {
                    match last {
                        Elem::Int(v) => *v = *v*10 + (c as i16 - '0' as i16),
                        Elem::List(v) => {
                            v.push(Elem::Int(c as i16 - '0' as i16));
                            level += 1;
                            last = v.last_mut().unwrap();
                        },
                        Elem::None if level == 0 => {
                            level += 1;
                            *last = Elem::Int(c as i16 - '0' as i16)
                        },
                        Elem::None => panic!("cannot happen")
                    }
                },
                _ => panic!("unexpected character: {}", c)
            }
        }
        if let Elem::None = l {
            self.l1 = l;
            self.i += 1;
        } else if let Elem::None = self.l1 {
            self.all.push(l.clone());
            self.l1 = l;
        } else {
            if self.l1 < l {
                self.acc1 += self.i
            }
            self.all.push(l);
        }
    }

    fn after(&mut self) {
        self.all.sort();
        for (i, e) in self.all.iter().enumerate() {
            if self.m2 == *e {
                self.acc2 *= i as u16 + 1
            } else if self.m6 == *e {
                self.acc2 *= i as u16 + 1
            }
        }
    }
}

fn main() {
    aoc::run(&mut Task13::new())
}
