pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

impl Rectangle { 
    pub fn area(&self) -> u32 {
        self.width * self.height
    }
}

pub fn main() {
    let rec1 = Rectangle {
        width: 30,
        height:40,
    };
    
    println!("The area of the rectangle is {} square pixels.", rec1.area());
}