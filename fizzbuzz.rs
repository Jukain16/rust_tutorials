fn main() {
    for i in 0..101 {
            match (i % 3, i % 5) { // use match instead of if else for conciseness
                (0, 0) => println!("FizzBuzz"),
                (0, _) => println!("Fizz"),
                (_, 0) => println! ("Buzz"),
                (_, _) => println!("{}", i)
            }
    }
}
