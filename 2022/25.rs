#[allow(unused_macros)]
#[macro_use]
mod aoc;
use std::str::FromStr;

#[derive(Copy,Clone,Debug)]
struct SNAFU(i64);

impl FromStr for SNAFU {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut n = 0;
        for c in s.chars() {
            n = 5*n + match c {
                '2' => 2,
                '1' => 1,
                '0' => 0,
                '-' => -1,
                '=' => -2,
                _   => return Err(()),
            }
        }
        Ok(SNAFU(n))
    }
}

impl SNAFU {
    fn to_str(&self) -> String {
        let mut n = self.0;
        let mut s = String::from("");
        while n > 0 {
            let digit;
            (n, digit) = match n % 5 {
                0 => (n/5, '0'),
                1 => ((n-1)/5, '1'),
                2 => ((n-2)/5, '2'),
                3 => ((n+2)/5, '='),
                4 => ((n+1)/5, '-'),
                _ => panic!("cannot happen"),
            };
            s = digit.to_string() + &s
        }
        s
    }
}

struct Task25 {
    sum: SNAFU
}

impl Task25 {
    fn new() -> Task25 {
        Task25 {
            sum: SNAFU(0)
        }
    }
}

impl aoc::AdventurerOfCode for Task25 {
    fn handle_line(&mut self, line: String) {
        self.sum.0 += line.parse::<SNAFU>().unwrap().0;
    }

    fn after(&mut self) {
    }
}

aocfmt!{Task25, self, self.sum.to_str()}
aocmain!{Task25}
