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
    let price: f32 = input_line().trim().parse().unwrap();
    for i in 1..=15 {
        let total_price = (price * 10.) as u64 * i;
        let weight = i*100;
        let width = 4;
        println!("{weight:>width$} grams: {} UAH {} kop", total_price/100, total_price%100); 
    }
} 