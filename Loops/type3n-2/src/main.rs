use std::io;

fn main() {
    solution();
}

fn input_line() -> String {
    let mut input_line = String::new();
    let _ = io::stdin().read_line(&mut input_line);
    input_line
}

fn solution() {
    let num: u16 = input_line().trim().parse().unwrap();
    for i in 1..=num {
        print!("{} ", i*3-2); 
    }
}