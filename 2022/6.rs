fn main() {
    let mut line: String = String::new();
    match std::io::stdin().read_line(&mut line) {
        Ok(_) => print_marker(line),
        Err(_) => (),
    }
}

fn print_marker(line: String) {
    println!("{}", get_marker(&line, 4));
    println!("{}", get_marker(&line, 14));
}

fn get_marker(line: &String, size: usize) -> usize {
    let mut i = size;
    loop {
        let mut unique = true;
        let mut bits = 0u32;
        for cx in line[i-size..i].chars() {
            let bit = cx as u8 - 'a' as u8;
            if bits & (1 << bit) != 0 {
                unique = false;
            } else {
                bits |= 1 << bit
            }
        }
        if unique {
            return i;
        }
        i += 1
    }
}
