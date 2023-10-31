use std::io;

fn main() {
    println!("{}",solution());
}

fn input_line() -> String {
    let mut input_line = String::new();
    let _ = io::stdin().read_line(&mut input_line);
    input_line
}

fn solution() -> u64 {
    let cards: Vec<u64> = input_line()
            .split_whitespace()
            .map(|q| q.parse().unwrap())
            .collect();
    let sum_expected = cards[0]*(cards[0]+1)/2;
    let mut sum = 0;
    for card in 1..cards.len() { sum += cards[card]; }
    sum_expected - sum
} 