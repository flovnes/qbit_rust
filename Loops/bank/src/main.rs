use std::io;

fn main() {
    println!("{}", solution());
}

fn input_line() -> String {
    let mut input_line = String::new();
    let _ = io::stdin().read_line(&mut input_line);
    input_line
}

fn input_values() -> (u64,u64,u64) {
    let values: Vec<u64> = input_line()
    .split_whitespace()
    .map(|q| q.parse().unwrap())
    .collect();
    (values[0], values[1], values[2])
}

fn solution() -> u64 {
    let (current, interest, end) = input_values();  
    let mut counter = 0;
    let mut cents: u16 = 0;
    let mut dollars = current as f64;
    let mut change;
    loop {
        if dollars >= end as f64 { return counter }
        counter +=1;
        change = (dollars + (cents as f64 / 100.0)) / 100.0 * interest as f64;

        // println!("dollars on [{}]: {}", counter-1, dollars);
        // println!("change: {}", change);
        // println!("add int to dollars: {}", change.floor());
        
        dollars += change.floor();
        change -= change.floor();

        // println!("dollars on [{}]: {}", counter-1, dollars);

        // println!("add float to cents: {}", (change * 100.).floor() as u16);

        cents += (change * 100.).floor() as u16;

        // println!("float part: {}", cents);
        
        dollars += (cents/100) as f64;
        cents %= 100;

        // println!("[{counter}] {0}.{1:02}", dollars, cents);
    }
} 