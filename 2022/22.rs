#[allow(unused_macros)]
#[macro_use]
mod aoc;
extern crate nalgebra;
use nalgebra::base::{Matrix,DMatrix,VecStorage};
use nalgebra::base::dimension::Dynamic;

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
    Left,
    Up,
    Right,
    Down,
}

impl Direction {
    fn ordinal(&self) -> usize {
        match *self {
            Direction::Left => 0,
            Direction::Up => 1,
            Direction::Right => 2,
            Direction::Down => 3,
        }
    }
    fn from_ordinal(ord: usize) -> Self {
        match ord {
            0 => Direction::Left,
            1 => Direction::Up,
            2 => Direction::Right,
            3 => Direction::Down,
            _ => panic!("bad ordinal"),
        }
    }
    fn right(&self) -> Self {
        Direction::from_ordinal((self.ordinal() + 1)%4)
    }
    fn opposite(&self) -> Self {
        Direction::from_ordinal((self.ordinal() + 2)%4)
    }
    fn left(&self) -> Self {
        Direction::from_ordinal((self.ordinal() + 3)%4)
    }
}

#[derive(Clone,Copy,Debug,PartialEq)]
enum Face {
    Front,
    Back,
    Up,
    Down,
    Left,
    Right
}

impl Face {
    fn ordinal(&self) -> usize {
        match *self {
            Face::Up => 0,
            Face::Front => 1,
            Face::Down => 2,
            Face::Back => 3,
            Face::Left => 4,
            Face::Right => 5,
        }
    }
    fn from_ordinal(ord: usize) -> Self {
        match ord {
            0 => Face::Up,
            1 => Face::Front,
            2 => Face::Down,
            3 => Face::Back,
            4 => Face::Left,
            5 => Face::Right,
            _ => panic!("bad ordinal"),
        }
    }
}

#[derive(Clone,Copy,Debug)]
struct CubeFace {
    face: Face,
    neighbors: [Face;4],
    init: bool,
    base: (usize, usize),
}

impl CubeFace {
    fn new(face: Face) -> Self {
        CubeFace {
            face: face,
            neighbors: match face {
                Face::Up => [Face::Left, Face::Back, Face::Right, Face::Front],
                Face::Front => [Face::Left, Face::Up, Face::Right, Face::Down],
                Face::Down => [Face::Left, Face::Front, Face::Right, Face::Back],
                Face::Back => [Face::Left, Face::Down, Face::Right, Face::Up],
                Face::Left => [Face::Back, Face::Up, Face::Front, Face::Down],
                Face::Right => [Face::Front, Face::Up, Face::Back, Face::Down],
            },
            init: false,
            base: (0, 0)
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
    cube: [CubeFace;6],
    start: Position,
    start_init: bool,
    dir: Direction,
    acc1: usize,
    acc2: usize,
}

impl Task22 {
    fn new() -> Task22 {
        Task22 {
            reading_map: true,
            lines: Vec::new(),
            instructions: Vec::new(),
            map: DMatrix::from_element(0, 0, Cell::Edge),
            cube_side: 0,
            cube: [CubeFace::new(Face::from_ordinal(0)),
                   CubeFace::new(Face::from_ordinal(1)),
                   CubeFace::new(Face::from_ordinal(2)),
                   CubeFace::new(Face::from_ordinal(3)),
                   CubeFace::new(Face::from_ordinal(4)),
                   CubeFace::new(Face::from_ordinal(5))],
            start: (None,0,0),
            start_init: false,
            dir: Direction::Right,
            acc1: 0,
            acc2: 0,
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
        //println!("{:?} {:?}", pos, dir);
        match instr {
            Instruction::TurnLeft => *dir = dir.left(),
            Instruction::TurnRight => *dir = dir.right(),
            Instruction::Go(n) => for _ in 0..n {
                self.go(pos, dir)
            },
        }
    }

    fn go(&self, pos: &mut Position, dir: &mut Direction) {
        match *pos {
            (None, x, y) => {
                let (mut newx, mut newy) = (x, y);
                loop {
                    match *dir {
                        Direction::Up => {
                            newx = if newx == 0 {
                                self.map.nrows() - 1
                            } else {
                                newx - 1
                            }
                        },
                        Direction::Down => {
                            newx = if newx + 1 == self.map.nrows() {
                                0
                            } else {
                                newx + 1
                            }
                        },
                        Direction::Left => {
                            newy = if newy == 0 {
                                self.map.ncols() - 1
                            } else {
                                newy - 1
                            }
                        },
                        Direction::Right => {
                            newy = if newy + 1 == self.map.ncols() {
                                0
                            } else {
                                newy + 1
                            }
                        },
                    }
                    match self.map[(newx,newy)] {
                        Cell::Empty => *pos = (None, newx, newy),
                        Cell::Wall => (),
                        Cell::Edge => continue,
                    }
                    break
                }
            },
            (Some(cubeface), x, y) => {
                let (xmod, ymod) = (x % self.cube_side, y % self.cube_side);
                let wrap = match dir {
                    Direction::Up => xmod == 0,
                    Direction::Down => xmod + 1 == self.cube_side,
                    Direction::Left => ymod == 0,
                    Direction::Right => ymod + 1 == self.cube_side,
                };
                let (newface, newx, newy, newdir) = if wrap {
                    let newface = self.cube[cubeface.neighbors[dir.ordinal()].ordinal()];
                    let fromdirord = newface.neighbors.iter().position(|f|*f==cubeface.face).unwrap();
                    let newdir = Direction::from_ordinal(fromdirord).opposite();
                    let (newxmod, newymod) = match (dir.clone(), newdir) {
                        (Direction::Up, Direction::Up) => (self.cube_side - 1, ymod),
                        (Direction::Up, Direction::Down) => (0, self.cube_side - 1 - ymod),
                        (Direction::Up, Direction::Left) => (self.cube_side - 1 - ymod, self.cube_side - 1),
                        (Direction::Up, Direction::Right) => (ymod, 0),
                        (Direction::Right, Direction::Up) => (self.cube_side - 1, xmod),
                        (Direction::Right, Direction::Down) => (0, self.cube_side - 1 - xmod),
                        (Direction::Right, Direction::Left) => (self.cube_side - 1 - xmod, self.cube_side - 1),
                        (Direction::Right, Direction::Right) => (xmod, 0),
                        (Direction::Down, Direction::Up) => (self.cube_side - 1, self.cube_side - 1 - ymod),
                        (Direction::Down, Direction::Down) => (0, ymod),
                        (Direction::Down, Direction::Left) => (ymod, self.cube_side -1),
                        (Direction::Down, Direction::Right) => (self.cube_side - 1 - ymod, 0),
                        (Direction::Left, Direction::Up) => (self.cube_side - 1, self.cube_side - 1 - xmod),
                        (Direction::Left, Direction::Down) => (0, xmod),
                        (Direction::Left, Direction::Left) => (xmod, self.cube_side - 1),
                        (Direction::Left, Direction::Right) => (self.cube_side - 1 - xmod, 0),
                    };
                    //println!("transition! {:?}", (newface, newxmod + newface.base.0, newymod + newface.base.1, newdir));
                    (newface, newxmod + newface.base.0, newymod + newface.base.1, newdir)
                } else {
                    match dir {
                        Direction::Up => (cubeface, x-1, y, *dir),
                        Direction::Down => (cubeface, x+1, y, *dir),
                        Direction::Left => (cubeface, x, y-1, *dir),
                        Direction::Right => (cubeface, x, y+1, *dir),
                    }
                };
                match self.map[(newx,newy)] {
                    Cell::Empty => {
                        *pos = (Some(newface), newx, newy);
                        *dir = newdir
                    },
                    Cell::Wall => (),
                    Cell::Edge => panic!("cannot happen"),
                }
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
                            if ! self.start_init {
                                self.start.2 = j;
                                self.start_init = true
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

        // define the cube
        self.cube_side = ((total/6) as f32).sqrt() as usize;
        self.cube[Face::Front.ordinal()].base = (self.start.1, self.start.2);
        self.cube[Face::Front.ordinal()].init = true;
        let mut todo = vec![Face::Front];
        while let Some(face) = todo.pop() {
            assert!(self.cube[face.ordinal()].init);
            let (x,y) = self.cube[face.ordinal()].base;
            let cube_side = self.cube_side.clone();
            let test: [(bool, Box<dyn Fn(usize, usize) -> (usize, usize)>, Direction);4] =
                [(y >= self.cube_side, Box::new(|x,y|(x, y-cube_side)), Direction::Left),
                (y + self.cube_side < columns, Box::new(|x,y|(x, y+cube_side)), Direction::Right),
                (x >= self.cube_side, Box::new(|x,y|(x-cube_side, y)), Direction::Up),
                (x + self.cube_side < rows, Box::new(|x,y|(x+cube_side, y)), Direction::Down)];
            for (condition, nextfn, direction) in test {
                if condition && self.map[nextfn(x, y)] != Cell::Edge {
                    let nextface = self.cube[face.ordinal()].neighbors[direction.ordinal()];
                    let nextcubeface = &mut self.cube[nextface.ordinal()];
                    if nextcubeface.init {
                        //println!("{:?} at {:?} should be {:?} of {:?}", nextface, nextcubeface.base, direction, face);
                        assert_eq!(face, nextcubeface.neighbors[direction.opposite().ordinal()]);
                    } else {
                        nextcubeface.init = true;
                        nextcubeface.base = nextfn(x, y);
                        while nextcubeface.neighbors[direction.opposite().ordinal()] != face {
                            nextcubeface.neighbors.swap(0, 1);
                            nextcubeface.neighbors.swap(1, 2);
                            nextcubeface.neighbors.swap(2, 3);
                        }
                        //println!("{:?} is at {:?} is {:?} of {:?}", nextface, nextcubeface.base, direction, face);
                        //for i in 0..4 {
                        //    println!("  {:?} is {:?} of {:?}", nextcubeface.neighbors[i], Direction::from_ordinal(i), nextface);
                        //}
                        todo.push(nextface);
                    }
                }
            }
        }

        // walk the map (task 1)
        let mut pos = self.start.clone();
        let mut dir = self.dir.clone();
        for instr in self.instructions.clone() {
            self.exec(&mut pos, &mut dir, instr)
        }
        self.acc1 = 1000*(pos.1+1) + 4*(pos.2+1) + match dir {
            Direction::Right => 0,
            Direction::Down => 1,
            Direction::Left => 2,
            Direction::Up => 3,
        };

        // walk the map (task 2)
        let mut pos = self.start.clone();
        pos.0 = Some(self.cube[Face::Front.ordinal()]);
        let mut dir = self.dir.clone();
        for instr in self.instructions.clone() {
            self.exec(&mut pos, &mut dir, instr)
        }
        self.acc2 = 1000*(pos.1+1) + 4*(pos.2+1) + match dir {
            Direction::Right => 0,
            Direction::Down => 1,
            Direction::Left => 2,
            Direction::Up => 3,
        }
    }
}

aocfmt!{Task22, self, self.acc1, self.acc2}
aocmain!{Task22}
