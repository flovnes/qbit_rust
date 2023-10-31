use std::io;

fn main() {
    let (sum, max) = solution();
    println!("{} {}", sum, max);
}

fn input_line() -> String {
    let mut input_line = String::new();
    let _ = io::stdin().read_line(&mut input_line);
    input_line
}

fn solution() -> (u64, u64) {
    let mut num: u64 = input_line().trim().parse().unwrap();
    let mut sum = 0;
    let mut max = num;
    loop {
        match num%2 {
            0 => {num /= 2; sum += num},
            _ => {num = num*3+1; max = max.max(num)}
        }
        if num == 1 {break}
    }
    (sum, max)
}