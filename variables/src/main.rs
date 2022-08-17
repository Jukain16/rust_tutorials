fn main() {
    let  x = 5;
    let x = x + 1;
    let x = x * 2; // shadows and add the values of x each time
    println!("The value of x is: {}", x);
}
