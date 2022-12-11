mod aoc;

struct Task10 {
    cur: i32,
    step: u16,
    acc1: i32,
    acc2: Vec<bool>,
}

enum Cmd {
    Noop,
    Addx(i32)
}

impl Task10 {
    fn new() -> Task10 {
        Task10 {
            cur: 1,
            step: 1,
            acc1: 0,
            acc2: vec![false; 240],
        }
    }

    fn check_step(&mut self) {
        if self.step % 40 == 20 {
            self.acc1 += self.step as i32*self.cur
        }
        let crtpos: i32 = (self.step as i32 - 1) % 40;
        if crtpos >= self.cur - 1 && crtpos <= self.cur + 1 {
            self.acc2[self.step as usize - 1] = true
        }
    }

    fn handle_noop(&mut self) {
        self.check_step();
        self.step += 1;
    }

    fn handle_addx(&mut self, v: i32) {
        for _ in 0..2 {
            self.check_step();
            self.step += 1
        };
        self.cur += v
    }

    fn handle_cmd(&mut self, cmd: Cmd) {
        match cmd {
            Cmd::Addx(v) => self.handle_addx(v),
            Cmd::Noop => self.handle_noop(),
        }
    }

    fn task1(&self) -> i32 {
        self.acc1
    }
}

impl std::fmt::Display for Task10 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut res = write!(f, "{}\n", self.task1());
        for (i, c) in self.acc2.iter().enumerate() {
            if i % 40 == 0 {
                res = res.and_then(|_| write!(f, "\n"))
            }
            if *c {
                res = res.and_then(|_| write!(f, "#"))
            } else {
                res = res.and_then(|_| write!(f, "."))
            }
        }
        res
    }
}

impl aoc::AdventurerOfCode for Task10 {
    fn handle_line(&mut self, line: String) {
        let cmd: Cmd = if line == "noop" {
            Cmd::Noop
        } else {
            let (cmd, value) = line.split_once(" ").unwrap();
            if cmd == "addx" {
                Cmd::Addx(value.parse().unwrap())
            } else {
                panic!("unknown command {}", line)
            }
        };
        self.handle_cmd(cmd)
    }

    fn after(&mut self) {
        self.check_step()
    }
}

fn main() {
    aoc::run(&mut Task10::new())
}
