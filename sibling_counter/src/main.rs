use std::io;
pub fn siblings(s: String) {
    println!("Enter the number of siblings you have.");
    let mut siblings = String::new();
    io::stdin().read_line(&mut siblings)
        .expect("Failed to read line.");
    let siblings: i32 = siblings.trim().parse()
        .expect("Not a number!");
    println!("Hello {},you have {} siblings", s, siblings);
}
fn main() {
    println!("Hello, please enter your name.");
    let mut name = String::new();
    io::stdin().read_line(&mut name)
        .expect("Failed to read line.");
    siblings(name);
}
