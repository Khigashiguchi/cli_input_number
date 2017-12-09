use std::io;
use std::io::Write;

fn main() {
    println!("Insert your first number: ");
    let _ = io::stdout().flush();

    let mut first_input = String::new();
    let _ = io::stdin().read_line(&mut first_input);
    let first_input_num: i32 = first_input.trim().parse().unwrap();
    println!("Your number is {}", first_input_num);

    print!("Insert your second number: ");
    let _ = io::stdout().flush();

    let mut second_input = String::new();
    let _ = io::stdin().read_line(&mut second_input);
    let second_input_num: i32 = second_input.trim().parse().unwrap();
    println!("Your number is {}", second_input_num);
}
