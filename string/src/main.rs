fn main() {
    let mut s = String::from("hello");    
    s.push_str(", world"); // ush_str() appends a literal to a String

    println!("{}", s);
}
