fn main() {
    for i in 0..101 {
        match (i % 3, i % 5) {
            (0,0) => println!("FizzBuzz"),
            (0, _) => println!("Fizz"),
            (_, 0) => println!("Buzz"),
            (_,_) => println!("{}", i)
        }
    }
}
