#![feature(linked_list_cursors)]
#[allow(unused_macros)]
#[macro_use]
mod aoc;
use std::collections::LinkedList;

struct Task20 {
    file: LinkedList<(usize, i64)>,
    i: usize,
    acc1: i64,
    acc2: i64,
}

impl Task20 {
    fn new() -> Task20 {
        Task20 {
            file: LinkedList::new(),
            i: 0,
            acc1: 0,
            acc2: 0,
        }
    }

    fn mix(file: &mut LinkedList<(usize, i64)>, times: u8, mult: i64) {
        let filelen = file.len();
        for _ in 0..times {
            for i in 0..filelen {
                let mut cursor = file.cursor_front_mut();
                loop {
                    match cursor.current() {
                        Some((j, _)) if i == *j => break,
                        _                       => cursor.move_next(),
                    }
                }
                if let Some(v) = cursor.remove_current() {
                    if let None = cursor.current() {
                        cursor.move_next();
                    }
                    let mut n = (v.1 * mult) % (filelen-1) as i64;
                    if n >= 0 {
                        while n > 0 {
                            cursor.move_next();
                            if let Some(_) = cursor.current() {
                                n -= 1;
                            }
                        }
                        cursor.insert_before(v);
                    } else if n < 0 {
                        while n < 0 {
                            cursor.move_prev();
                            if let Some(_) = cursor.current() {
                                n += 1;
                            }
                        }
                        cursor.insert_before(v);
                    }
                }
            }
        }
    }

    fn get_coord(file: &LinkedList<(usize, i64)>) -> i64 {
        let mut cursor = file.cursor_front();
        while let Some((_, n)) = cursor.current() {
            if *n != 0 {
                cursor.move_next();
            } else {
                break;
            }
        }
        let mut sum = 0;
        for _ in 0..3 {
            let mut moveby = 1000 % file.len();
            while moveby > 0 {
                cursor.move_next();
                if let Some(_) = cursor.current() {
                    moveby -= 1;
                }
            }
            if let Some((_, x)) = cursor.current() {
                sum += x
            }
        }
        sum
    }
}

impl aoc::AdventurerOfCode for Task20 {
    fn handle_line(&mut self, line: String) {
        let n: i64 = line.parse().unwrap();
        self.file.push_back((self.i, n));
        self.i += 1;
    }

    fn after(&mut self) {
        let mut file1 = self.file.clone();
        Task20::mix(&mut file1, 1, 1);
        self.acc1 = Task20::get_coord(&file1);
        let mut file2 = self.file.clone();
        let mult = 811589153;
        Task20::mix(&mut file2, 10, mult);
        self.acc2 = mult * Task20::get_coord(&file2);
    }
}

aocfmt!{Task20, self, self.acc1, self.acc2}
aocmain!{Task20}
