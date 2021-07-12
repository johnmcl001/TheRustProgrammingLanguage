// fn multiply(first_number: &str, second_number: &str) -> i32 {
// 	let first_number: i32 = first_number.parse().unwrap();
// 	let second_number: i32 = second_number.parse().unwrap();
// 	first_number + second_number
// }

// fn main() {
// 	let twenty = multiply("10", "2");
// 	println!("double is {}", twenty);
// 	let tt = multiply("t", "2");
// 	println!("double is {}", tt);
// }

use std::num::ParseIntError;

fn main() -> Result<(), ParseIntError> {
	let number = "10";
	let number: i32 = match number.parse() {
		Ok(number) => number,
		Err(e) => return Err(e),
	};
	println!("{}", number);
	Ok(())
}