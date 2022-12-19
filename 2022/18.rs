#[allow(unused_macros)]
#[macro_use]
mod aoc;

struct Task18 {
    max:(usize,usize,usize),
    droplet: Vec<Vec<Vec<bool>>>,
    acc1: u16,
    trapped: Vec<(usize,usize,usize)>,
    acc2: u16,
}

use Neighbour::*;
enum Neighbour {
    Xminus,
    Xplus,
    Yminus,
    Yplus,
    Zminus,
    Zplus,
}
const NEIGHBOURS: [Neighbour;6] = [Xminus, Xplus, Yminus, Yplus, Zminus, Zplus];

impl Task18 {
    fn new() -> Task18 {
        Task18 {
            max: (0,0,0),
            droplet: Vec::new(),
            acc1: 0,
            trapped: Vec::new(),
            acc2: 0,
        }
    }

    fn get_neighbour(&self, x: usize, y: usize, z: usize, n: &Neighbour
        ) -> Option<(usize,usize,usize)>
    {
        match n {
            Xminus =>
                if x > 0 {
                    Some((x-1,y,z))
                } else {
                    None
                },
            Xplus =>
                if x < self.max.0-1 {
                    Some((x+1,y,z))
                } else {
                    None
                },
            Yminus =>
                if y > 0 {
                    Some((x,y-1,z))
                } else {
                    None
                },
            Yplus =>
                if y < self.max.1-1 {
                    Some((x,y+1,z))
                } else {
                    None
                },
            Zminus =>
                if z > 0 {
                    Some((x,y,z-1))
                } else {
                    None
                },
            Zplus =>
                if z < self.max.2-1 {
                    Some((x,y,z+1))
                } else {
                    None
                },
        }
    }

    fn is_trapped(&self, x: usize, y: usize, z: usize,
        seen: &mut Vec<(usize,usize,usize)>) -> bool
    {
        if seen.contains(&(x,y,z)) {
            return true;
        }
        seen.push((x,y,z));
        for nei in &NEIGHBOURS {
            match self.get_neighbour(x, y, z, nei) {
                None => return false,
                Some((xn,yn,zn)) =>
                    if ! self.droplet[xn][yn][zn] && ! self.is_trapped(xn,yn,zn,seen) {
                        return false
                    },
            }
        }
        return true
    }

    fn check_neighbour(&mut self, i: usize, j: usize, k: usize, n: &Neighbour) {
        match self.get_neighbour(i,j,k,n) {
            None => {
                self.acc1 += 1;
                self.acc2 += 1;
            },
            Some((x,y,z)) => {
                if ! self.droplet[x][y][z] {
                    self.acc1 += 1;
                    if ! self.trapped.contains(&(x,y,z)) {
                        let mut maybe_trapped = Vec::new();
                        if ! self.is_trapped(x,y,z,&mut maybe_trapped) {
                            self.acc2 += 1;
                        } else {
                            self.trapped.extend(maybe_trapped.into_iter());
                        }
                    }
                }
            }
        }
    }
}

impl aoc::AdventurerOfCode for Task18 {
    fn handle_line(&mut self, line: String) {
        let coord: Vec<&str> = line.split(",").collect();
        let x: usize = coord[0].parse().unwrap();
        let y: usize = coord[1].parse().unwrap();
        let z: usize = coord[2].parse().unwrap();
        if x >= self.droplet.len() {
            self.droplet.resize(x+1, Vec::new());
            self.max.0 = self.max.0.max(x+1);
        }
        if y >= self.droplet[x].len() {
            self.droplet[x].resize(y+1, Vec::new());
            self.max.1 = self.max.1.max(y+1);
        }
        if z >= self.droplet[x][y].len() {
            self.droplet[x][y].resize(z+1, false);
            self.max.2 = self.max.2.max(z+1);
        }
        self.droplet[x][y][z] = true;
    }

    fn after(&mut self) {
        // make uniform size for simpler conditions
        for i in 0..self.droplet.len() {
            self.droplet[i].resize(self.max.1, Vec::new());
            for j in 0..self.droplet[i].len() {
                self.droplet[i][j].resize(self.max.2, false);
            }
        }
        for i in 0..self.droplet.len() {
            for j in 0..self.droplet[i].len() {
                for k in 0..self.droplet[i][j].len() {
                    if self.droplet[i][j][k] {
                        for n in &NEIGHBOURS {
                            self.check_neighbour(i,j,k,n);
                        }
                    }
                }
            }
        }
    }
}

aocfmt!{Task18, self, self.acc1, self.acc2}
aocmain!{Task18}
