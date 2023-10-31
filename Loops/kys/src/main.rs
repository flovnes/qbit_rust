use std::io;

fn main() {
    solution();
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

fn solution() {
    let (uah, kop) = input_values();
    for i in 1..=10 {
        let total_price = (uah * 100 + kop) * i;
        let width = 2;
        println!("{i:>width$} kg: {0},{1:02} UAH", total_price/100, total_price%100); 
    }
}