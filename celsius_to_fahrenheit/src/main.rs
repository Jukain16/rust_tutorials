// Converts Celsius from Fahrenheit and vice versa
// Celsius to Fahrenheit = celsius*9/5 + 32
// Fahrenheit to Celsius = (fahrenheit - 32) / 9/5 
use std::io; // input library
fn main() { // main function
    println!("Celsius or Fahrenheit? Enter C or F"); // asks for celsius or fahrenheit
    let mut temperature = String::new(); // assigns input to temperature
    io::stdin().read_line(&mut temperature) // uses input library
        .expect("Failed to read line."); // failure line
    let temperature: char = temperature.trim().parse() // converts to character
        .expect("Not a character");
    if temperature == 'c'|| temperature == 'C'  { // if user chooses celsius
        println!("Enter temperature in Celsius"); // asks for temp
        let  mut celsius = String::new(); // assigns input to temp
        io::stdin().read_line(&mut celsius) 
            .expect("Failed to read line");
        let celsius: u32 = celsius.trim().parse() // converts to integer
            .expect("Not a number");
        let  fahrenheit = celsius * 9 / 5 + 32; // calculates temperature in fahrenheit
        println!("The temperature in degrees Fahrenheit is: {}", fahrenheit); // pritns temp
    }
    else if temperature == 'f' || temperature == 'F' { // same but with fahrenheit to celsius
        println!("Enter temperature in Fahrenheit");
        let mut  fahrenheit = String::new();
        io::stdin().read_line(&mut fahrenheit)
            .expect("Failed to read line");
        let fahrenheit: u32 = fahrenheit.trim().parse()
            .expect("Not a number");
        let  celsius = (fahrenheit - 32) * 5 / 9;
        println!("The temperature in degrees Celsius is: {}", celsius);
    }
    else { // for if user tries to be funny
        println!("Invalid entry. Please enter a number.");
    }
}
