use std::io;

fn main() {
    solution();
}

fn input_line() -> String {
    let mut input_line = String::new();
    let _ = io::stdin().read_line(&mut input_line);
    input_line
}

fn solution()  {
    let num: u8 = input_line().trim().parse().unwrap();
    for line in 1..=num*2-1 {
        // print!("line {line}: ");
        if line <= num {
            for i in 1..=num {
                 if i > num-line {
                    if i == num { print!("*") }
                    else if i == num-line+1 { print!("* ") }
                    else { print!("+ ") }
                } else { print!("  ") }
            }
        } else {
            for i in 1..=num {
                if i > line-num {
                    if i == num { print!("*") }
                    else if i == line-num+1 { print!("* ") }
                    else { print!("+ ") }
                } else { print!("  ") }
            }
        }
        print!("\n");
    }
} 