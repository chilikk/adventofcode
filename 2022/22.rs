#[allow(unused_macros)]
#[macro_use]
mod aoc;
extern crate nalgebra;
use nalgebra::base::{Matrix,DMatrix,VecStorage};
use nalgebra::base::dimension::Dynamic;
use std::collections::BTreeMap;

#[derive(Clone,PartialEq,Debug)]
enum Cell {
    Edge,
    Empty,
    Wall,
}

#[derive(Clone,Copy,Debug)]
enum Instruction {
    Go(usize),
    TurnLeft,
    TurnRight
}

#[derive(Clone,Copy,Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone,Copy,Debug)]
enum CubeFace {
    Front,
    Back,
    Up,
    Down,
    Left,
    Right
}

impl CubeFace {
    fn ordinal(&self) -> u8 {
        match *self {
            CubeFace::Up => 0,
            CubeFace::Front => 1,
            CubeFace::Down => 2,
            CubeFace::Back => 3,
            CubeFace::Left => 4,
            CubeFace::Right => 5,
        }
    }
    fn from_ordinal(ord: u8) -> Self {
        match ord {
            0 => CubeFace::Up,
            1 => CubeFace::Front,
            2 => CubeFace::Down,
            3 => CubeFace::Back,
            4 => CubeFace::Left,
            5 => CubeFace::Right,
            _ => panic!("bad ordinal"),
        }
    }
}

type Position = (Option<CubeFace>, usize, usize);

struct Task22 {
    reading_map: bool,
    lines: Vec<String>,
    instructions: Vec<Instruction>,
    map: Matrix<Cell,Dynamic,Dynamic,VecStorage<Cell,Dynamic,Dynamic>>,
    cube_side: usize,
    cube: Vec<Matrix<Cell,Dynamic,Dynamic,VecStorage<Cell,Dynamic,Dynamic>>>,
    start: Position,
    dir: Direction,
    mode: Mode,
    acc: usize,
}

impl Task22 {
    fn new(mode: Mode) -> Task22 {
        Task22 {
            reading_map: true,
            lines: Vec::new(),
            instructions: Vec::new(),
            map: DMatrix::from_element(0, 0, Cell::Edge),
            cube_side: 0,
            cube: Vec::new(),
            start: (None,0,0),
            dir: Direction::Right,
            mode: mode,
            acc: 0,
        }
    }

    fn instr_char(&mut self, cur_number: &mut Option<usize>, c: char) {
        match (*cur_number, c) {
            (None, 'L') =>
                self.instructions.push(Instruction::TurnLeft),
            (None, 'R') =>
                self.instructions.push(Instruction::TurnRight),
            (Some(n), 'L'|'R') => {
                self.instructions.push(Instruction::Go(n));
                *cur_number = None;
                self.instr_char(&mut None, c)
            },
            (None, '0'..='9') =>
                *cur_number = Some(c as usize - '0' as usize),
            (Some(n), '0'..='9') =>
                *cur_number = Some(n*10 + c as usize - '0' as usize),
            (_, _) =>
                panic!("invalid char '{}'", c)
        }
    }

    fn exec(&mut self, pos: &mut Position, dir: &mut Direction, instr: Instruction) {
        println!("{:?} {:?}", pos, dir);
        match instr {
            Instruction::TurnLeft => match *dir {
                Direction::Left => *dir = Direction::Down,
                Direction::Down => *dir = Direction::Right,
                Direction::Right => *dir = Direction::Up,
                Direction::Up => *dir = Direction::Left,
            },
            Instruction::TurnRight => match *dir {
                Direction::Left => *dir = Direction::Up,
                Direction::Up => *dir = Direction::Right,
                Direction::Right => *dir = Direction::Down,
                Direction::Down => *dir = Direction::Left,
            },
            Instruction::Go(n) => for _ in 0..n {
                match *dir {
                    Direction::Up => self.go_up(pos, dir),
                    Direction::Down => self.go_down(pos, dir),
                    Direction::Left => self.go_left(pos, dir),
                    Direction::Right => self.go_right(pos, dir),
                }
            },
        }
    }

    fn go_up(&self, pos: &mut Position, dir: &mut Direction) {
        if let (None, x, y) = *pos {
            let mut newx = x;
            loop {
                newx = if newx == 0 {
                    self.map.nrows() - 1
                } else {
                    newx - 1
                };
                match self.map[(newx,y)] {
                    Cell::Empty => pos.1 = newx,
                    Cell::Wall => (),
                    Cell::Edge => continue,
                }
                break
            }
        }
    }
    fn go_down(&self, pos: &mut Position, dir: &mut Direction) {
        if let (None, x, y) = *pos {
            let mut newx = x;
            loop {
                newx = if newx + 1 == self.map.nrows() {
                    0
                } else {
                    newx + 1
                };
                match self.map[(newx,y)] {
                    Cell::Empty => pos.1 = newx,
                    Cell::Wall => (),
                    Cell::Edge => continue,
                }
                break
            }
        }
    }
    fn go_left(&self, pos: &mut Position, dir: &mut Direction) {
        if let (None, x, y) = *pos {
            let mut newy = y;
            loop {
                newy = if newy == 0 {
                    self.map.ncols() - 1
                } else {
                    newy - 1
                };
                match self.map[(x,newy)] {
                    Cell::Empty => pos.2 = newy,
                    Cell::Wall => (),
                    Cell::Edge => continue,
                }
                break
            }
        }
    }
    fn go_right(&self, pos: &mut Position, dir: &mut Direction) {
        if let (None, x, y) = *pos {
            let mut newy = y;
            loop {
                newy = if newy + 1 == self.map.ncols() {
                    0
                } else {
                    newy + 1
                };
                match self.map[(x,newy)] {
                    Cell::Empty => pos.2 = newy,
                    Cell::Wall => (),
                    Cell::Edge => continue,
                }
                break
            }
        }
    }
}

impl aoc::AdventurerOfCode for Task22 {
    fn handle_line(&mut self, line: String) {
        if self.reading_map {
            if line.len() > 0 {
                self.lines.push(line);
            } else {
                self.reading_map = false
            }
        } else {
            let mut cur_number: Option<usize> = None;
            for c in line.chars() {
                self.instr_char(&mut cur_number, c)
            }
            if let Some(n) = cur_number {
                self.instructions.push(Instruction::Go(n))
            }
        }
    }

    fn after(&mut self) {
        let mut total = 0;
        let rows = self.lines.len();
        let columns = self.lines.iter().fold(0, |acc,line|acc.max(line.len()));
        self.map = DMatrix::from_element(rows, columns, Cell::Edge);
        for (i, line) in self.lines.iter().enumerate() {
            for (j, c) in line.chars().enumerate() {
                match c {
                    ' ' => (),
                    '.' => {
                        total += 1;
                        self.map[(i,j)] = Cell::Empty;
                        if i == 0 {
                            if let (_, 0, 0) = self.start {
                                self.start.2 = j
                            }
                        }
                    },
                    '#' => {
                        total += 1;
                        self.map[(i,j)] = Cell::Wall;
                    },
                    _ => panic!("unexpected character {}", c),
                }
            }
        }
        if let Mode::Subtask2 = self.mode {
            self.cube_side = total/6;
            self.cube.resize(6, DMatrix::from_element(
                    self.cube_side, self.cube_side, Cell::Edge));
            let pos = self.start.clone();
            let mut i = 0;
            while i < self.map.nrows() {
                let mut j = 0;
                while j < self.map.ncols() {
                    match self.map[(i,j)] {
                        Cell::Edge => (),
                        _ => {
                            let face = m

                        },
                    }
                    j += 50
                }
                i += 50
            }
        }
        let mut pos = self.start.clone();
        let mut dir = self.dir.clone();
        for instr in self.instructions.clone() {
            self.exec(&mut pos, &mut dir, instr)
        }
        self.acc = 1000*(pos.1+1) + 4*(pos.2+1) + match dir {
            Direction::Right => 0,
            Direction::Down => 1,
            Direction::Left => 2,
            Direction::Up => 3,
        }
    }
}

aocfmt!{Task22, self, self.acc}
aocsubtasks!{Task22}
