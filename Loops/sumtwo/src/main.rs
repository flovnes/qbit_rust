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
    let mut sum = 0;
    let mut previous_element = i16::MAX;
    loop {
        let current_element: i16 = input_line().trim().parse().unwrap();
        if current_element == 0 && previous_element == 0 { break }
        sum += current_element as i64;
        previous_element = current_element;
    }
    sum
}