// struct Rectangle (u32, u32);
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rectangle = Rectangle{
        width: 30, 
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rectangle)
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.height * rectangle.width
}
