use std::io;

fn main() {
    println!("{}", solution());
}

fn input_line() -> String {
    let mut input_line = String::new();
    let _ = io::stdin().read_line(&mut input_line);
    input_line
}

fn solution() -> i64 {  
    let mut num: i64 = input_line().trim().parse().unwrap();
    num = num.abs();
    let mut counter = 0;
    while num > 0 {
        counter += num % 10 % 2;
        num /= 10;
    }
    counter
} 