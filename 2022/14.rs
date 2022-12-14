#[macro_use]
mod aoc;

#[derive(Clone)]
struct Point{x: usize, y: usize}
struct Path(Point, Point);

struct Task14 {
    mode: Mode,
    paths: Vec<Path>,
    sand: Point,
    min: Point,
    max: Point,
    size: Point,
    map: Vec<char>,
    grains: u32,
}

impl Task14 {
    fn new(m: Mode) -> Task14 {
        let sand = Point{x: 500, y: 0};
        Task14 {
            mode: m,
            paths: Vec::new(),
            sand: sand.clone(),
            min: sand.clone(),
            max: sand.clone(),
            size: Point{x: 0, y: 0},
            map: Vec::new(),
            grains: 0,
        }
    }

    fn index(&self, p: &Point) -> usize {
        (p.y-self.min.y)*self.size.x+p.x-self.min.x
    }

    fn sandfall(&mut self) -> bool {
        let mut sand = self.sand.clone();
        loop {
            let index = self.index(&sand);
            if self.map[index] != '.' {
                break false
            } else if index + self.size.x < self.map.len() {
                if self.map[index + self.size.x] == '.' {
                    sand.y += 1
                } else if index % self.size.x == 0 {
                    break false
                } else if self.map[index + self.size.x - 1] == '.' {
                    sand.x -= 1;
                    sand.y += 1
                } else if index % self.size.x == self.size.x - 1 {
                    break false
                } else if self.map[index + self.size.x + 1] == '.' {
                    sand.x += 1;
                    sand.y += 1
                } else {
                    self.map[index] = 'o';
                    break true
                }
            } else {
                break false
            }
        }

    }

    fn print_map(&self) {
        for (i, c) in self.map.iter().enumerate() {
            if i % self.size.x == 0 {
                println!("")
            };
            print!("{c}")
        }
    }
}

aocfmt!{Task14, self, self.grains}

impl aoc::AdventurerOfCode for Task14 {
    fn handle_line(&mut self, line: String) {
        let mut prev: Option<Point> = None;
        let mut cur: Point;
        for s in line.split(" -> ") {
            let (x,y) = s.split_once(",").unwrap();
            cur = Point{x: x.parse().unwrap(), y: y.parse().unwrap()};
            if cur.x < self.min.x {
                self.min.x = cur.x;
            } else if cur.x > self.max.x {
                self.max.x = cur.x;
            }
            if cur.y < self.min.y {
                self.min.y = cur.y;
            } else if cur.y > self.max.y {
                self.max.y = cur.y;
            }
            match prev {
                None => prev = Some(cur),
                Some(point) => {
                    self.paths.push(Path(point, cur.clone()));
                    prev = Some(cur)
                },
            }
        }
    }

    fn after(&mut self) {
        if let Mode::Subtask2 = self.mode {
            self.max.y += 2;
            self.min.x = self.min.x.min(self.sand.x+self.min.y-self.max.y-1);
            self.max.x = self.max.x.max(self.sand.x+self.max.y-self.min.y+1);
            self.paths.push(Path(Point{x: self.min.x, y: self.max.y},
                                 Point{x: self.max.x, y: self.max.y}));

        }
        self.size = Point{x: self.max.x-self.min.x+1, y: self.max.y-self.min.y+1};
        self.map = vec!['.'; self.size.x*self.size.y];
        for path in self.paths.iter() {
            if path.0.x == path.1.x {
                let x = path.0.x;
                for y in path.0.y.min(path.1.y)..path.0.y.max(path.1.y)+1 {
                    let index = self.index(&Point{x: x,y: y});
                    self.map[index] = '#';
                }
            } else if path.0.y == path.1.y {
                let y = path.0.y;
                for x in path.0.x.min(path.1.x)..path.0.x.max(path.1.x)+1 {
                    let index = self.index(&Point{x: x, y: y});
                    self.map[index] = '#';
                }

            }
        }
        while self.sandfall() {
            self.grains += 1
        }
        //self.print_map()
    }
}

aocsubtasks!{Task14}
