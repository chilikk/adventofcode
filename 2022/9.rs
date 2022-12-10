mod aoc;
use std::cmp::Ordering;

#[derive(Copy, Clone, PartialOrd, Eq, PartialEq)]
struct Point {
    x: i16,
    y: i16
}

impl Ord for Point {
    fn cmp(&self, other: &Point) -> Ordering {
        if self.x < other.x || self.x == other.x && self.y < other.y {
            Ordering::Less
        } else if self.x == other.x && self.y == other.y {
            Ordering::Equal
        } else {
            Ordering::Greater
        }
    }
}

impl Point {
    fn mv(&mut self, dir: char) {
        match dir {
            'R' => self.x += 1,
            'L' => self.x -= 1,
            'U' => self.y -= 1,
            'D' => self.y += 1,
            _ => ()
        }
    }

    fn follow(&mut self, head: &Point) {
        if head.y == self.y + 2 {
            self.y += 1;
            if self.x < head.x {
                self.x += 1;
                assert!((head.x-self.x).abs() <= 1);
            } else if self.x > head.x {
                self.x -= 1;
                assert!((head.x-self.x).abs() <= 1);
            } else {
                assert_eq!(self.x, head.x)
            }
        } else if head.y <= self.y - 2 {
            self.y -= 1;
            if self.x < head.x {
                self.x += 1;
                assert!((head.x-self.x).abs() <= 1);
            } else if self.x > head.x {
                self.x -= 1;
                assert!((head.x-self.x).abs() <= 1);
            } else {
                assert_eq!(self.x, head.x)
            }
        } else if head.x >= self.x + 2 {
            self.x += 1;
            if self.y < head.y {
                self.y += 1;
                assert!((head.y-self.y).abs() <= 1);
            } else if self.y > head.y {
                self.y -= 1;
                assert!((head.y-self.y).abs() <= 1);
            } else {
                assert_eq!(self.y, head.y)
            }
        } else if head.x <= self.x - 2 {
            self.x -= 1;
            if self.y < head.y {
                self.y += 1;
                assert!((head.y-self.y).abs() <= 1);
            } else if self.y > head.y {
                self.y -= 1;
                assert!((head.y-self.y).abs() <= 1);
            } else {
                assert_eq!(self.y, head.y)
            }
        } else {
            assert!((head.x-self.x).abs() <= 1 && (head.y-self.y).abs() <= 1)
        }
    }
}

struct Task9 {
    head: Point,
    tails1: Vec<Point>,
    intermediates: [Point; 9],
    tails2: Vec<Point>
}

impl Task9 {
    fn new() -> Task9 {
        Task9 {
            head: Point{x: 0, y: 0},
            tails1: vec![Point{x: 0, y: 0}],
            intermediates: [Point{x: 0, y:0}; 9],
            tails2: vec![Point{x: 0, y: 0}]
        }
    }

    fn task1(&self) -> usize {
        let mut tails1 = self.tails1.clone();
        tails1.sort_unstable();
        tails1.dedup();
        tails1.len()
    }

    fn task2(&self) -> usize {
        let mut tails2 = self.tails2.clone();
        tails2.sort_unstable();
        tails2.dedup();
        tails2.len()
    }
}

impl std::fmt::Display for Task9 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\n{}", self.task1(), self.task2())
    }
}

impl aoc::AdventurerOfCode for Task9 {
    fn handle_line(&mut self, line: String) {
        let (dirstr, stepsstr) = line.split_once(" ").unwrap();
        let dir = dirstr.chars().next().unwrap();
        let steps: u8 = stepsstr.parse().unwrap();
        for _ in 0..steps {
            self.head.mv(dir);
            self.intermediates[0].follow(&self.head);
            self.tails1.push(self.intermediates[0].clone());
            for i in 1..9 {
                let (prev, this) = self.intermediates.split_at_mut(i);
                this[0].follow(&prev[i-1])
            }
            self.tails2.push(self.intermediates[8].clone())
        }
    }

    fn after(&mut self) {
    }
}

fn main() {
    aoc::run(&mut Task9::new())
}
