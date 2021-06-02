#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// Unit struct
struct Unit;

// tuple struct
struct Pair(i32, f32);

// struct with 2 fields
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

// Structs can be reused as fields of another struct
#[allow(dead_code)]
#[derive(Debug)]
struct Rectangle {
    // A rectangle specified by where top left and bottom right are
    top_left: Point,
    bottom_right: Point,
}

fn rect_area(rectangle: Rectangle) -> f32 {
    (rectangle.bottom_right.x - rectangle.top_left.x) * (rectangle.top_left.y - rectangle.bottom_right.y)
}

fn square(point: Point, length: f32) -> Rectangle {
    let x = point.x;
    let y = point.y;
    Rectangle{
        top_left: point,
        bottom_right: Point{
            x: x + length,
            y: y + length,
        },
    }
}

fn main() {
    // Create struct with field init shorthand
    let name = String::from("Peter");
    let age = 27;
    let peter = Person{ name, age };

    // Print debug struct
    println!("{:?}", peter);

    // Instantiate a point
    let point: Point = Point{ x: 10.3, y: 0.4 };

    // Access fields of the point
    println!("point coordinates: ({}, {})", point.x, point.y);

    // Make a new pont by using struct updatesyntax ot use fields of existing
    let bottom_right = Point{ x: 5.2, ..point };

    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    // Destructure the point using let binding
    let Point {x: top_edge, y: left_edge } = point;

    let rectangle = Rectangle{
        // struct instantiation is an expression too
        top_left: Point { x: left_edge, y: top_edge },
        bottom_right: bottom_right,
    };

    // instantiate a unit struct
    let _unit = Unit;

    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);

    // Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1); 

    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);

    println!("The area of the rectangle is {}", rect_area(rectangle));
    println!("Square: {:?}", square(point, 32.0));
}