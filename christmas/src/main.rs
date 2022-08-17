// Prints out the lyrics to 'The 12 days of Christmas'
fn main() {
    let days = ["first", "second", "third", "fourth", "fifth","sixth", "seventh", "eigth", "ninth", "tenth", "eleventh", "twelfth",];
    let gifts = [
        "a partrdige in a pear tree", 
        "two turtle doves", 
        "three french hens",
        "four calling birds",
        "five golden rings", 
        "six geese-a-laying", 
        "seven swans-a-swimming", 
        "eight maids-a-milking",
        "nine ladies dancing", 
        "ten lords-a-leaping", 
        "eleven pipers piping", 
        "twelve drummers drumming"
    ];
    
    for day in 0..12 {
        println!("On the {} day of Christmas, my true love gave to me," , days[day]);
        for index in (0..day + 1).rev() {
            if index == 0 && day > 0 {
                print!("And");
            }
            print!(" {} ", gifts[index]);
        }
    }
}
