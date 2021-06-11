// globals are declared outside all other scopes
static LANGUAGE: &str = "Rust";
const THRESHOLD: i32 = 10;

fn is_big(n: i32) -> bool {
	// access constant in some function
	n > THRESHOLD
}

fn main() {
	let n = 16;

	// access constant in the main thread
	println!("this is {}", LANGUAGE);
	println!("the threshold is {}", THRESHOLD);
	println!("{} is {}", n, if is_big(n) { "big" } else { "small" });

	// Error cannot modify a constant
	// THRESHOLD = 5;
}