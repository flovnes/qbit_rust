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
    let count = input_line().trim().parse().unwrap();
    for _ in 0..count {
        println!("Hello!"); 
    }
}