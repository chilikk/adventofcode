mod aoc;

struct TaskN {
}

impl TaskN {
    fn new() -> TaskN {
        TaskN {
        }
    }

    fn task1(&self) -> u16 {
        0
    }

    fn task2(&self) -> u16 {
        0
    }
}

impl std::fmt::Display for TaskN {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\n{}", self.task1(), self.task2())
    }
}

impl aoc::AdventurerOfCode for TaskN {
    fn handle_line(&mut self, line: String) {
    }

    fn after(&mut self) {
    }
}

fn main() {
    aoc::run(&mut TaskN::new())
}
