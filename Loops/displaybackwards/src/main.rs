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
    let mut num: i64 = input_line().trim().parse().unwrap();
    if num < 0 { print!("-") }
    num = num.abs();
    let mut rev = 0;
    while num > 0 {
        rev = rev * 10 + num % 10;
        num /= 10;
    }
    print!("{rev}");
} 