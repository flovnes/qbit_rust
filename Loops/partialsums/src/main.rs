use std::io;

fn main() {
    let (a,b) = solution();
    println!("{a} {b}");
}

fn input_line() -> String {
    let mut input_line = String::new();
    let _ = io::stdin().read_line(&mut input_line);
    input_line
}

fn solution() -> (u64, u64) {
    let num: f64 = input_line().trim().parse().unwrap();
    let mut sum = 0.0;
    let mut max = 0;
    let mut min = 0;

    const MAX_N: u64 = 1248397;
    for i in 1..=MAX_N {
        sum += 1./i as f64;
        if ((sum * 1000000.0) as u64) < ((num * 1000000.0) as u64) { max = i }
        if ((sum * 1000000.0) as u64) > ((num * 1000000.0) as u64) { min = i; break}
    }
    (max, min)
}