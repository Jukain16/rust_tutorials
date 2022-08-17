fn main() {
    let mut s = String::from("Hello world"); 
    let slice = &s[..2]; // equal to [0..2]
    let word = first_word(&s); // word will get the value of s
    s.clear(); // empties the string, making it equal to ""
}

fn first_word(s: &str) -> &str  {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
