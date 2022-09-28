use std::env; //reads the values of command line arguments
fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Hello, world!");
}
