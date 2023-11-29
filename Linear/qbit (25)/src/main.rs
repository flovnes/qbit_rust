use std::io;

fn main() {

    println!("{}", solution(input_values()));
}

fn input_line() -> String {
    let mut input_line = String::new();
    let _ = io::stdin().read_line(&mut input_line);
    input_line
}

fn input_values() -> u64 {
    input_line().trim().parse().unwrap()
}

fn solution(day_year: u64) -> u64 { 
    ((day_year + 5)-1)%7 + 1
}