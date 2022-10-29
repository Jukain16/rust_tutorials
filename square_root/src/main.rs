use std::io;
    pub fn my_sqrt(x: f64) -> f64 {
        let sqrt = x.sqrt();
        println!("{}", sqrt);
        return sqrt;
    }
fn main() {
    let mut num = String::new();
    io::stdin().read_line(&mut num)
        .expect("Failed to read line");
    let num: f64 = num.trim().parse()
        .expect("Not a number");
    my_sqrt(num);
}
