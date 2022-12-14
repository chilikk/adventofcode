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
    print!("{adv}")
}

macro_rules! aocfmt {
    ($s: ty, $self:ident$(, $x: expr)*) => {
        impl std::fmt::Display for $s {
            fn fmt(&$self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let mut res = std::fmt::Result::Ok(());
                $(
                    res = res.and(write!(f, "{}\n", $x));
                )*
                res
            }
        }
    }
}

macro_rules! aocmain {
    ($s: ty) => {
        fn main() {
            aoc::run(&mut $s::new())
        }
    }
}

macro_rules! aocsubtasks {
    ($s: ty) => {
        enum Mode {
            Subtask1,
            Subtask2,
        }

        fn main() {
            let args = std::env::args().collect::<Vec<String>>();
            if args.len() < 2 {
                println!("indicate submode as arg, e.g. `./aoc 1`");
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
            aoc::run(&mut <$s>::new(mode))
        }
    }
}
