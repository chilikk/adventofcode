pub trait AdventurerOfCode: std::fmt::Display {
    fn handle_line(&mut self, line: String);
    fn after(&mut self);
}

pub fn run(adv: &mut dyn AdventurerOfCode) {
    for line in std::io::stdin().lines() {
        match line {
            Ok(linestr) => adv.handle_line(linestr),
            Err(_) => (),
        }
    }
    adv.after();
    println!("{adv}")
}
