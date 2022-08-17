// Generates the nth number in the fibonacci sequence
// Fibonacci = sum of previous two numbers
use std::io;
fn main() {
    println!("Type exit to quit program");
    loop {
        println!("Type a positive number");
        let mut int = String::new();
        io::stdin().read_line(&mut int)
            .expect("Failed to read line");
        if int.trim() == "exit" {
            break;
        }
        let int: i32 =  match int.trim().parse() {
            Ok(int) => int,
            Err(_) => continue,
        };
        println!("fibonacci ({}) => {}", int, fib(int)); // prints fibonacci sequence 14 times
    }
}

fn fib(n: i32) -> i32 { // fibonacci function
                        // assign i32
    if n <= 0 { 
        return 0; // checks if value is equal to or less than 0 and returns 0 
    }
    else if n == 1 {
        return 1; // checks if value is equal to 1 and returns 1
    }
    else {
        return fib (n - 1) + fib(n - 2); // returns fibonacci sequence
    }
}
