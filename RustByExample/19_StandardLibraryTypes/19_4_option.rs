// an integer division that doesn't panic
fn checked_division(dividend: i32, divisor: i32) -> Option<i32> {
	if divisor == 0 {
		None // failure represented by None variant
	} else {
		Some(dividend / divisor) // Success represented by Some variant
	}
}

// function handles division that might not succeed
fn try_division(dividend: i32, divisor: i32) {
	// option values can be pattern matched
	match checked_division(dividend, divisor) {
		None => println!("{} / {} failed", dividend, divisor),
		Some(quotient) => println!("{} / {} = {}", dividend, divisor, quotient),
	}
}

fn main() {
	try_division(4, 2);
	try_division(1, 0);

	// binding None to a variable needs to be type annotated
	let none: Option<i32> = None;
	let _equivalen_none = None::<i32>;

	let optional_float = Some(0f32);

	// Unwrapping a some variant will extract the value wrapped
	println!("{:?} unwraps to {:?}", optional_float, optional_float.unwrap());

	// Unwrapping a none variant will cause Rust to panic
	// println!("{:?} unwraps to {:?}", none, none.unwrap());
}