pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

pub fn main() {
    let rec1 = Rectangle {
        width: 30,
        height:40,
    };
    
    println!("The area of the rectangle is {} square pixels.", area(&rec1));
}

pub fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}