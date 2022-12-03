use std::io;

fn main() {
    let mut total1: u32 = 0;
    let mut total2: u32 = 0;
    for line in io::stdin().lines() {
        match line {
            Ok(linestr) => {
                let linesplit = linestr.split_once(" ");
                total1 += match linesplit {
                    Some((first, second)) => score1(first, second),
                    None => 0,
                };
                total2 += match linesplit {
                    Some((first, second)) => score2(first, second),
                    None => 0,
                }
            },
            Err(_) =>
                (),
        }
    }
    println!("{total1}");
    println!("{total2}");
}

fn score1(first: &str, second: &str) -> u32 {
    let firsti = match first {
        "A" => 0,
        "B" => 1,
        "C" => 2,
        &_ => return 0,
    };
    let secondi = match second {
        "X" => 0,
        "Y" => 1,
        "Z" => 2,
        &_ => return 0,
    };
    if firsti == secondi {
        return draw(secondi);
    } else if secondi == get_winning_i(firsti) {
        return win(secondi);
    } else {
        return lose(secondi)
    }
}

fn score2(first: &str, second: &str) -> u32 {
    let firsti = match first {
        "A" => 0,
        "B" => 1,
        "C" => 2,
        &_ => return 0,
    };
    match second {
        "X" => lose(get_losing_i(firsti)),
        "Y" => draw(firsti),
        "Z" => win(get_winning_i(firsti)),
        &_ => 0,
    }
}

fn lose(i: u32) -> u32          { i + 1 }
fn draw(i: u32) -> u32          { 3 + i + 1 }
fn win(i: u32) -> u32           { 6 + i + 1 }
fn get_winning_i(i: u32) -> u32 { (i + 1) % 3 }
fn get_losing_i(i: u32) -> u32  { (i + 2) % 3 }
