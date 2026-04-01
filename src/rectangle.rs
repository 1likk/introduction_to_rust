pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

impl Rectangle { 
    pub fn area(&self) -> u32 {
        self.width * self.height
    }

    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height 
    }
}

pub fn main() {
    let rec1 = Rectangle {
        width: 30,
        height:40,
    };

    let rec2 = Rectangle {
        width: 50,
        height: 60,
    };
    
    let rec3 = Rectangle { 
        width: 20,
        height: 30,
    };
    
    println!("The area of the rectangle is {} square pixels.", rec1.area());
    println!("The area of the rectangle is {} square pixels.", rec2.area());
    println!("The area of the rectangle is {} square pixels.", rec3.area());
    println!("Can rec1 hold rec2? {}", rec1.can_hold(&rec2));
    println!("Can rec1 hold rec3? {}", rec1.can_hold(&rec3));
}