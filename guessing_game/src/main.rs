use std::io; // std = standard library io= input/output
use std::cmp::Ordering; // compare and ordering 
use rand::Rng; // random number generator

fn main() {
    println!("Guess a number between 1 and 100!"); // prints message to guess
    let secret_number = rand::thread_rng().gen_range(1, 101); // generator inclusive to lower
                                                               // bound but exclusive to upper
                                                               // bound
    loop {
        println!("Please input your guess."); //  prints message to ask for guess then demands input
        let mut guess = String::new(); // assigns input to variable
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line"); // :: = associated function; .expect programs error into
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed: {}", guess); // {} hold the value in place 
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too large!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
