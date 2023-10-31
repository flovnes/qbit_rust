use std::io;

fn main() {
    println!("{}",solution());
}

fn input_line() -> String {
    let mut input_line = String::new();
    let _ = io::stdin().read_line(&mut input_line);
    input_line
}

fn input_values() -> (u16,u16) {
    let values: Vec<u16> = input_line()
    .split_whitespace()
    .map(|q| q.parse().unwrap())
    .collect();
    (values[0], values[1])
}

fn solution() -> u16 {
    let (floors, flats) = input_values();
    let mut counter = 0;
    for _floors in 1..=floors {
        let windows: Vec<u16> = input_line()
            .split_whitespace()
            .map(|q| q.parse().unwrap())
            .collect();
        for flat in 1..=flats {
            if windows[(2*flat-1) as usize] == 1 || windows[(2*(flat-1)) as usize] == 1 {
                counter += 1;
            }
            // counter += windows[(2*flat-1) as usize]|windows[(2*(flat-1)) as usize];
            // println!("{}",windows[(2*flat-1) as usize]|windows[(2*(flat-1)) as usize]);
        }
    }
    counter
} 