#[derive(Debug)] // allows to display structs
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle { // method used for structs; implementation block
                 // reads data in the struci, does not take ownership
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    fn square(size: u32) -> Rectangle { // associated function
        Rectangle {
            width: size,
            height: size
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    println!("Rectangle is {:#?}", rect1);
    println!("The area of the rectangle is {} square pixels.", rect1.area());
}
