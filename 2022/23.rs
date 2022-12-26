#[allow(unused_macros)]
#[macro_use]
mod aoc;
use std::collections::VecDeque;

struct Task23 {
    line: usize,
    elfcnt: usize,
    elfs: VecDeque<VecDeque<usize>>,
    acc1: usize,
    acc2: usize,
}

impl Task23 {
    const ORDER: [fn(&Task23, usize, usize) -> Option<(usize, usize)>;4] = [
        Task23::propose_north,
        Task23::propose_south,
        Task23::propose_west,
        Task23::propose_east,
    ];

    fn new() -> Task23 {
        Task23 {
            line: 0,
            elfcnt: 0,
            elfs: VecDeque::new(),
            acc1: 0,
            acc2: 0,
        }
    }

    fn propose_north(&self, i: usize, j: usize) -> Option<(usize,usize)> {
        if self.elfs[i-1][j-1] == 0 && self.elfs[i-1][j] == 0
            && self.elfs[i-1][j+1] == 0 {
            Some((i-1,j))
        } else {
            None
        }
    }

    fn propose_south(&self, i: usize, j: usize) -> Option<(usize,usize)> {
        if self.elfs[i+1][j-1] == 0 && self.elfs[i+1][j] == 0
            && self.elfs[i+1][j+1] == 0 {
            Some((i+1,j))
        } else {
            None
        }
    }

    fn propose_west(&self, i: usize, j: usize) -> Option<(usize,usize)> {
        if self.elfs[i-1][j-1] == 0 && self.elfs[i][j-1] == 0
            && self.elfs[i+1][j-1] == 0 {
            Some((i,j-1))
        } else {
            None
        }
    }

    fn propose_east(&self, i: usize, j: usize) -> Option<(usize,usize)> {
        if self.elfs[i-1][j+1] == 0 && self.elfs[i][j+1] == 0
            && self.elfs[i+1][j+1] == 0 {
            Some((i,j+1))
        } else {
            None
        }
    }

    fn propose_any(&self, i: usize, j: usize) -> bool {
        ! (self.elfs[i-1][j-1] == 0 && self.elfs[i-1][j] == 0
            && self.elfs[i-1][j+1] == 0 && self.elfs[i][j+1] == 0
            && self.elfs[i+1][j+1] == 0 && self.elfs[i+1][j] == 0
            && self.elfs[i+1][j-1] == 0 && self.elfs[i][j-1] == 0)
    }

    fn propose(&self, i: usize, j: usize, turn: usize) -> Option<(usize,usize)> {
        let mut proposal = None;
        if self.propose_any(i, j) {
            for o in 0..Task23::ORDER.len() {
                proposal = Task23::ORDER[(turn+o) % Task23::ORDER.len()](&self, i, j);
                if let Some(_) = proposal {
                    break
                }
            }
            proposal
        } else {
            None
        }
    }

    fn maybe_extend(&mut self) {
        if self.elfs[0].iter().any(|x| *x!=0) {
            self.elfs.push_front(VecDeque::from(vec![0;self.elfs[0].len()]))
        }
        if self.elfs[self.elfs.len()-1].iter().any(|x| *x!=0) {
            self.elfs.push_back(VecDeque::from(vec![0;self.elfs[0].len()]))
        }
        if self.elfs.iter().any(|row| row[0]!=0) {
            self.elfs.iter_mut().for_each(|row|row.push_front(0));
        }
        if self.elfs.iter().any(|row| row[row.len()-1]!=0) {
            self.elfs.iter_mut().for_each(|row|row.push_back(0));
        }
    }

    fn maybe_contract(&mut self) {
        while self.elfs[0].iter().all(|x| *x==0) {
            self.elfs.pop_front();
        }
        while self.elfs[self.elfs.len()-1].iter().all(|x| *x==0) {
            self.elfs.pop_back();
        }
        while self.elfs.iter().all(|row| row[0]==0) {
            self.elfs.iter_mut().for_each(|row|{row.pop_front();});
        }
        while self.elfs.iter().all(|row| row[row.len()-1]==0) {
            self.elfs.iter_mut().for_each(|row|{row.pop_back();});
        }
    }

    fn count_empty(&self) -> usize {
        self.elfs.iter().map(|row|row.iter().filter(|e|**e==0).count()).sum()
    }

    fn exec_turn(&mut self, turn: usize) -> bool {
        let mut proposals = Vec::new();
        // collect proposals
        for (i, row) in self.elfs.iter().enumerate() {
            for (j, &e) in row.iter().enumerate() {
                if e != 0 {
                    proposals.push((e,i,j,self.propose(i, j, turn)));
                }
            }
        }
        // eliminate duplicates
        proposals.sort_by(|(_,_,_,proposal1),(_,_,_,proposal2)|proposal1.cmp(proposal2));
        for i in 0..proposals.len()-1 {
            let mut next = i+1;
            if proposals[next].3 == proposals[i].3 {
                while proposals[next].3 == proposals[i].3 {
                    proposals[next].3 = None;
                    next += 1;
                    if next == proposals.len() {
                        break
                    }
                }
                proposals[i].3 = None;
            }
        }
        let mut did_move = false;
        // execute remaining proposals
        for (e,i,j,proposal) in proposals.iter() {
            if let Some((newi,newj)) = proposal {
                self.elfs[*newi][*newj] = *e;
                self.elfs[*i][*j] = 0;
                did_move = true;
            }
        }
        did_move
    }

    //fn print_map(&self) {
    //    for row in self.elfs.iter() {
    //        for &e in row.iter() {
    //            print!("{}", match e {
    //                0 => '.',
    //                _ => '#',
    //            })
    //        }
    //        println!();
    //    }
    //    println!();
    //}
}

impl aoc::AdventurerOfCode for Task23 {
    fn handle_line(&mut self, line: String) {
        self.elfs.push_back(VecDeque::with_capacity(line.len()));
        for c in line.chars() {
            match c {
                '.' => self.elfs[self.line].push_back(0),
                '#' => {
                    self.elfcnt += 1;
                    self.elfs[self.line].push_back(self.elfcnt);
                },
                _ => panic!("invalid char"),
            }
        }
        self.line += 1;
    }

    fn after(&mut self) {
        self.maybe_extend();
        let mut turn = 0;
        while self.exec_turn(turn) {
            turn += 1;
            if turn == 10 {
                self.maybe_contract();
                self.acc1 = self.count_empty();
            }
            self.maybe_extend();
        }
        self.acc2 = turn + 1;
    }
}

aocfmt!{Task23, self, self.acc1, self.acc2}
aocmain!{Task23}
