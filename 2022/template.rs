#[macro_use]
mod aoc;

struct TaskN {
}

impl TaskN {
    fn new() -> TaskN {
        TaskN {
        }
    }
}

aocfmt!{TaskN, self}

impl aoc::AdventurerOfCode for TaskN {
    fn handle_line(&mut self, line: String) {
    }

    fn after(&mut self) {
    }
}

fn main() {
    aoc::run(&mut TaskN::new())
}
