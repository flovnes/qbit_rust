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
    println!("{} very important numbers:", (start-end).abs()+1); 
    if start<end {
        for i in start..=end {
            println!("{i}"); 
        }
    } else {
        for i in (end..=start).rev() {
            println!("{i}"); 
        }
    }
}