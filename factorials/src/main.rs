use std::io;
fn factorial(num: u32) -> u32 {
    if num <= 1 {
        return 1;
    }
    num * factorial(num - 1);
}
fn main() {
    println!("Enter positive interger.");
    let mut num = String::new();
    io::stdin().read_line(&mut num)
        .expect("Failed to read line.");
    let num: u32 = num.trim().parse()
        .expect("Not a number.");
    println!("{}", factorial(num));
    }
