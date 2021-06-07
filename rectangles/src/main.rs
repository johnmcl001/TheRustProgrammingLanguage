// struct Rectangle (u32, u32);
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.area() > other.area()
    }
}

fn main() {
    let rectangle = Rectangle{
        width: 30, 
        height: 50,
    };

    println!("The area of the rectangle is {} square pixels.", rectangle.area());

    let rect1 = Rectangle { width: 30, height: 50 };    
    let rect2 = Rectangle { width: 10, height: 40 };   
    let rect3 = Rectangle { width: 60, height: 45 };    
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));   
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}

