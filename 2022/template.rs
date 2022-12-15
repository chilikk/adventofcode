#[allow(unused_macros)]
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

impl aoc::AdventurerOfCode for TaskN {
    fn handle_line(&mut self, line: String) {
    }

    fn after(&mut self) {
    }
}

aocfmt!{TaskN, self}
aocmain!{TaskN}
