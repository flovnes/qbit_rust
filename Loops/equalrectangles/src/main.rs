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
    let area: u32 = input_line().trim().parse().unwrap();
    for div in 1..=((area as f32).sqrt() as u32) {
        if area%div==0 {
            println!("{} {}", div, area/div);
        }
    }
}