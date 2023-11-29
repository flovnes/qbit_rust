use std::io;

fn main() {
    println!("{}", solution());
}

fn input_line() -> String {
    let mut input_line = String::new();
    let _ = io::stdin().read_line(&mut input_line);
    input_line
}

fn solution() -> u64 {  
    let mut num: u64 = input_line().trim().parse().unwrap();
    let mut counter = 0;
    while num > 0 {
        counter += num % 2;
        num /= 2;
    }
    counter
} 