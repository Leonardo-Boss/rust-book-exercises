use std::io;
use std::io::Write;

fn main() {
    println!("NTH FIBONACCI CALCULATOR:");
    print!("n = ");
    io::stdout().flush().unwrap();

    let mut n = String::new();
    io::stdin()
        .read_line(&mut n)
        .expect("Error reading");

    let n: u64 = n
        .trim()
        .parse()
        .expect("Input was not a number");
    
    let r = nth_fibonacci(n);
    println!("{n}th Fibonacci is {r}");
}

fn nth_fibonacci(n: u64) -> u64 {
    let mut n_minus_2 = 0;
    let mut n_minus_1 = 1;
    let mut temp: u64;
    for _ in 1..n {
        temp = n_minus_1; 
        n_minus_1 = n_minus_1 + n_minus_2;
        n_minus_2 = temp;
    }
    if n > 0 {n_minus_1} else {0}
}
