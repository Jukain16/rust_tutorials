use std::io;
pub fn roman_to_int(s: String) -> i32 {
    let s: i32 = s.trim().parse()
        .expect("Not a number");
    println!("{}", s);
    return s;
}
fn main() {
    let mut int = String::new();
    io::stdin().read_line(&mut int)
        .expect("Failed to read line.");
    roman_to_int(int);
}
