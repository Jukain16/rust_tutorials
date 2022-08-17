#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state); 25
        },
    }
}
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None, // matches are exhaustive and every possibility must be covered
                      // if let can be used if options do not need to be exhaustive
        Some(i) => Some(i + 1),
    }
} 
fn main(coin: Coin) {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    }
    else {
        count += 1;
    }

}
