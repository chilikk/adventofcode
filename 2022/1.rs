use std::cmp;
use std::io;

fn main() {
    let stdin = io::stdin();
    let mut max = 0;
    let mut maxvec: Vec<u32> = vec![];
    let mut acc = 0;

    for line in stdin.lines() {
        match line.unwrap().parse::<u32>() {
            Ok(v) => acc += v,
            Err(_) => {
                max = cmp::max(max, acc);
                maxvec.push(acc);
                maxvec.sort_by(|a,b| b.cmp(a));
                maxvec.truncate(3);
                acc = 0
            }
        }
    }
    max = cmp::max(max, acc);
    println!("{max}");
    maxvec.push(acc);
    maxvec.sort_by(|a,b| b.cmp(a));
    maxvec.truncate(3);
    let max3: u32 = maxvec.iter().sum();
    println!("{max3}");
}
