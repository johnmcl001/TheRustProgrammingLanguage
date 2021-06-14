struct Point {
	x: f64,
	y: f64,
}

// Implementation block, all Point methods go here
impl Point {
	// static method
	// don't need to be called by an instance
	// generally used as constructors
	fn origin() -> Point {
		Point { x: 0.0, y: 0.0 }
	}

	// Another static method taking 2 args
	fn new(x: f64, y: f64) -> Point {
		Point{ x, y }
	}
}

struct Rectangle {
	p1: Point,
	p2: Point,
}

impl Rectangle {
	// instance method
	// &self is syntactic sugar for self: &Self, where self is the sypt of the 
	// caller object, in this case Self = Rectangle
	fn area(&self) -> f64 {
		// self gives access to the struct fields via dot operator
		let Point { x: x1, y: y1 } = self.p1;
		let Point { x: x2, y: y2 } = self.p2;

		// abs is f64 method that returns the absolute value of the caller
		((x1 - x2) * (y2 - y1)).abs()
	}

	fn perimeter(&self) -> f64 {
		let Point { x: x1, y: y1 } = self.p1;
		let Point { x: x2, y: y2 } = self.p2;

		2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
	}

	// requires that the caller is mutable
	// &mut self desugars to self: &mut Self
	fn translate(&mut self, x: f64, y: f64) {
		self.p1.x += x;
		self.p2.x += x;
		self.p1.y += y;
		self.p2.y += y;
	}
}

// Pair owns resources: 2 heap allocated integers
struct Pair(Box<i32>, Box<i32>); 

impl Pair {
	// this method consumes teh resourcesof the caller object
	// self desugars to self: Self
	fn destroy(self) {
		// Destructure self
		let Pair(first, second) = self;

		println!("Destroying pair:({}, {})", first, second);

		// first and second go out of scope and are freed
	}
}

fn main() {
	let rectangle = Rectangle {
		// static methods are called using double colons
		p1: Point::origin(),
		p2: Point::new(3.0, 4.0),
	};

	// Instance methods are called using dot notation
	// note first arg &self is implicitly passed
	// rectangle.perimeter() === Rectangle::perimeter(&rectangle)
	println!("Rectangle perimeter: {}", rectangle.perimeter());
	println!("Rectangle area: {}", rectangle.area());

	let mut square = Rectangle {
		p1: Point::origin(),
		p2: Point::new(1.0, 1.0),
	};

	// Error: rectangle is immutable but this method requires a mutable object
	// rectangle.translate(1.0, 0.0);

	// mutable objects can call mutable methods
	square.translate(1.0, 1.0);

	let pair = Pair(Box::new(1), Box::new(2));

	pair.destroy();

	// Error: previous destroy call consumed pair
	// pair.destroy();
}