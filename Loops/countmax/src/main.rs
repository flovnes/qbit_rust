use std::io;
fn main() {
    println!("{}", solution());
}

fn input_line() -> String {
    let mut input_line = String::new();
    let _ = io::stdin().read_line(&mut input_line);
    input_line
}

fn solution() -> i16 {
    let mut max = i16::MIN;
    let mut counter = 0;
    loop {
        let num: i16 = input_line().trim().parse().unwrap();
        if num == 0 {break}
        if num > max { 
            max = num;
            counter = 1;     
        } else if num==max { counter +=1; }
    }
    counter
}