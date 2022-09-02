use std::io;
fn main() {
    println!("Do you want to convert to kilometres or miles? Enter k or m");
    let mut distance = String::new();
    io::stdin().read_line(&mut distance)
        .expect("Failed to read line.");
    let distance: char = distance.trim().parse()
        .expect("Not a character.");
    if distance == 'k'  || distance == 'K' {
        println!("Enter distance in miles");
        let mut miles = String::new();
        io::stdin().read_line(&mut miles)
            .expect("Failed to read line.");
        let miles: f64 = miles.trim().parse()
            .expect("Not a number.");
        let  kilometres = miles / 0.621371;
        println!("The distance is {} km.", kilometres);
    }
    else if distance == 'm' || distance == 'M' {
        println!("Enter distance in kilometres.");
        let mut kilometres = String::new();
        io::stdin().read_line(&mut kilometres)
            .expect("Failed to read line.");
        let kilometres: f64 = kilometres.trim().parse()
            .expect("Not a number.");
        let miles = kilometres * 0.621371;
        println!("The distance is {} miles.", miles);
    }
    else {
        println!("Enter kilometres or miles.");
    }
}
