use std::io;

fn main() {
    solution();
}

fn input_line() -> String {
    let mut input_line = String::new();
    let _ = io::stdin().read_line(&mut input_line);
    input_line
}

fn input_values() -> (f64,f64,f64) {
    let values: Vec<f64> = input_line()
    .split_whitespace()
    .map(|q| q.parse().unwrap())
    .collect();
    (values[0], values[1], values[2])
}

fn solution() {
    // let (current, interest, end) = input_values();
    let mut i: f64 = 1.0;
    let mut j: f64 = 1.0;
    let mut k: f64 = 1.0;
    loop {
        let (mut current, interest, end) = (i,j,k);
        let mut counter = 0;
        while current < end {
            current += (current * interest)  / 100.;
            counter +=1;
        }
        let math_counter = ((end/current).log10()/(1.0+interest/100.0).log10()).ceil() as u64;
        if counter != math_counter {
            println!("{current} {interest} {end}; {counter} != {math_counter}")
        }
        if j == 100.0 {
            j = 0.0;
        }
        i +=1.;
        j +=1.;
        k +=3.;
        if k > 1000000000. {
            break;
        }
    }
    // println!("{}", counter.ceil());
}