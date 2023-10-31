use std::io;

fn main() {
    solution();
}

fn input_line() -> String {
    let mut input_line = String::new();
    let _ = io::stdin().read_line(&mut input_line);
    input_line
}

fn solution()  {
    let num: u64 = input_line().trim().parse().unwrap();
    let mut position = 0;
    let numbers: Vec<u16> = input_line()
            .split_whitespace()
            .map(|q| q.parse().unwrap())
            .collect();
    for line in 0..num {
        for _ in 0..=line {
            print!("{} ",numbers[position as usize]);
            position+=1;
        }
        println!();
    }
}