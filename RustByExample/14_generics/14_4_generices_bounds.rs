// fn printer<T: display>(t: T) {
// 	println!("{}", t);
// }

// struct S<T: Display>(T);

// // Error: Vec<T> does not implement Display, this fails
// let s = S(vec![1]);

// trait which implementes the print marker
use std::fmt::Debug;

trait HasArea {
	fn area(&self) -> f64;
}

impl HasArea for Rectangle {
	fn area(&self) -> f64 {
		self.length * self.height
	}
}

#[derive(Debug)]
struct Rectangle {
	length: f64,
	height: f64,
}
#[allow(dead_code)]
struct Triangle { 
	length: f64, 
	height: f64
}

// The generic T must implement Debug, regardless of the type, this will work correctly
fn print_debug<T: Debug>(t: &T) {
	println!("{:?}", t);
}

// T must  implement HasArea. Any type which meets the bound can access HasArea in the function
fn area<T: HasArea>(t: &T) -> f64 {
	t.area()
}

fn main() {
	let rectangle = Rectangle{ length: 3.0, height: 4.0 };
	let triangle = Triangle{ length: 3.0, height: 4.0 };

	print_debug(&rectangle);
	println!("Area: {}", area(&rectangle));

	// // Errors since triangle doesn't implement debug or HasArea
	// print_debug(&triangle);
	// println!("Area: {}", area(&triangle));
}