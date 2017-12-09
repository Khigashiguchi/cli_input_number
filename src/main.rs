use std::io;
use std::io::Write;

fn main() {
    println!("Insert your first number: ");
    io::stdout().flush();

    let mut a = String::new();
    io::stdin().read_line(&mut a);
    let mut a: i32 = a.trim().parse().unwrap();

    print!("Insert your second number: ");
    io::stdout().flush();

    let mut b = String::new();
    io::stdin().read_line(&mut b);
    let mut b: i32 = b.trim().parse().unwrap();
}
