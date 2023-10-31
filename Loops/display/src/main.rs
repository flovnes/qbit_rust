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
    let mut negative = false;
    if num < 0 { negative = true; }
    num = num.abs();
    
    println!("***");
    if negative { println!("*-*"); }
    print_digits(num as u64);
    println!("***");
}

fn print_digits(num: u64) {
    if num < 10 {
        println!("*{}*", num%10);
        return;
    }
    print_digits(num/10);
    println!("*{}*", num%10);
}