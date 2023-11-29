use std::io;

fn main() {
    solution()
}

fn input_line() -> String {
    let mut input_line = String::new();
    let _ = io::stdin().read_line(&mut input_line);
    input_line
}

fn input_values() -> (i64, i64) {
    let values: Vec<i64> = input_line()
    .split_whitespace()
    .map(|q| q.parse().unwrap())
    .collect();
    (values[0], values[1])
}

fn solution() { 
    let matches = input_line().trim().parse().unwrap();
    let mut wins = 0;
    let mut total = 0;
    for _ in 0..matches {
        let (a_score, b_score) = input_values();
        total = total + a_score - b_score;
        match a_score {
            a if a > b_score => {wins+=1}, 
            a if a < b_score => {wins-=1},
            _ => () 
        }
    }
    match wins {
        x if x > 0 => println!("Andrey"),
        x if x < 0 => println!("Pasha"),
        _ => {
            match total {
                x if x > 0 => println!("Andrey"),
                x if x < 0 => println!("Pasha"),
                _ => println!("Draw")
            }
        }
    }
}