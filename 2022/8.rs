mod aoc;
use std::ops::Range;

#[derive(Clone)]
struct Tree {
    height: u8,
    visible: bool
}

struct Task8 {
    map: Vec<Vec<Tree>>
}

enum Mode {
    FromLeft,
    FromRight,
    FromTop,
    FromBottom,
}

enum LookMode {
    Right,
    Left,
    Up,
    Down,
}

impl Task8 {
    fn new() -> Task8 {
        return Task8 {
            map: Vec::new()
        };
    }

    fn mark_visible(&mut self,
                    range1: Range<usize>,
                    range2: Range<usize>,
                    mode: Mode) {
        let r1end = range1.end;
        let r2end = range2.end;
        for i in range1 {
            let mut visible = -1i8;
            for j in range2.clone() {
                let t = match mode {
                    Mode::FromLeft => &mut self.map[i][j],
                    Mode::FromTop => &mut self.map[j][i],
                    Mode::FromRight => &mut self.map[r1end-i-1][r2end-j-1],
                    Mode::FromBottom => &mut self.map[r2end-j-1][r1end-i-1],
                };
                if t.height as i8 > visible {
                    visible = t.height as i8;
                    t.visible = true
                }
            }
        }
    }

    fn look(&self, i: usize, jrange: Range<usize>, h: u8, mode: LookMode) -> u32 {
        let mut visible = 0u32;
        let jstart = jrange.start;
        let jend = jrange.end;
        for j in jrange {
            let t = match mode {
                LookMode::Right => &self.map[i][j],
                LookMode::Left => &self.map[i][jend-j-1+jstart],
                LookMode::Down => &self.map[j][i],
                LookMode::Up => &self.map[jend-j-1+jstart][i],
            };
            visible += 1;
            if t.height >= h {
                break
            }
        }
        visible
    }

    fn get_score(&self, i: usize, j: usize) -> u32 {
        let h = self.map[i][j].height;
        let jmax = self.map.len();
        let imax = self.map[0].len();
        let rightscore = self.look(i, j+1..jmax, h, LookMode::Right);
        let leftscore = self.look(i, 0..j, h, LookMode::Left);
        let downscore = self.look(j, i+1..imax, h, LookMode::Down);
        let upscore = self.look(j, 0..i, h, LookMode::Up);
        rightscore * leftscore * downscore * upscore
    }

    fn n_visible(&self) -> u16 {
        let mut n = 0u16;
        for v in self.map.iter() {
            for t in v.iter() {
                if t.visible {
                    n += 1
                }
            }
        }
        n
    }

    fn task1(&self) -> u16 {
        self.n_visible()
    }

    fn task2(&self) -> u32 {
        let mut score = 0;
        for i in 0..self.map.len() {
            for j in 0..self.map[i].len() {
                let curscore = self.get_score(i, j);
                if curscore > score {
                    score = curscore;
                }
            }
        }
        score
    }
}

impl std::fmt::Display for Task8 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\n{}", self.task1(), self.task2())
    }
}

impl aoc::AdventurerOfCode for Task8 {
    fn handle_line(&mut self, line: String) {
        if self.map.len() == 0 {
            // first line
            self.map.resize(line.len(), Vec::with_capacity(line.len()))
        }
        for (i, c) in line.chars().enumerate() {
            self.map[i].push(Tree{
                height: c as u8 - '0' as u8,
                visible: false
            })
        }
    }

    fn after(&mut self) {
        self.mark_visible(0..self.map.len(), 0..self.map[0].len(), Mode::FromLeft);
        self.mark_visible(0..self.map.len(), 0..self.map[0].len(), Mode::FromRight);
        self.mark_visible(0..self.map[0].len(), 0..self.map.len(), Mode::FromTop);
        self.mark_visible(0..self.map[0].len(), 0..self.map.len(), Mode::FromBottom);
    }
}

fn main() {
    aoc::run(&mut Task8::new())
}
