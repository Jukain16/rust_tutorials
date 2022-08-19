 fn main() {
    let v: Vec<i32> = Vec::new(); // stores more than one value in a single data structure
    let mut v = vec![1, 2, 3];
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8); // updates the vector
    let third: &i32 = &v[2];
    let does_not_exist = &v[100];
    let does_not_exist = v.get(100);
    println!("The third element is {}", third);
    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("THere is no third element."), // OPtion<&T>
    }
}
