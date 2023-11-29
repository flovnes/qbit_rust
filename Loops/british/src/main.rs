use std::io;

fn main() {
    solution();
}

fn input_line() -> String {
    let mut input_line = String::new();
    let _ = io::stdin().read_line(&mut input_line);
    input_line
}

fn input_values() -> (i64,i64) {
    let values: Vec<i64> = input_line()
    .split_whitespace()
    .map(|q| q.parse().unwrap())
    .collect();
    (values[0], values[1])
}

fn solution() {
    let (start, end) = input_values();
    for i in start..=end {
        println!("{}",(-1_i64).pow(i as u32 -1)*(3*i-1)); 
    }
}