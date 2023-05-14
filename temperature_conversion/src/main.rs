use std::io;
use std::io::Write;

fn main() {
    print!("temperature: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    
    let input: f32 = input
        .trim()
        .parse()
        .expect("Input was not a number");

    let fahrenheit = celsius_to_farenheit(input);
    let celsius = farenheit_to_celsius(input);

    println!("{input}C° = {fahrenheit}F°");
    println!("{input}F° = {celsius}C°");
}

fn celsius_to_farenheit(celsius: f32) -> f32 {
    celsius*1.8 + 32.
}

fn farenheit_to_celsius(fahrenheit: f32) -> f32 {
    (fahrenheit - 32.)/1.8
}
