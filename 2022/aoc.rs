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
