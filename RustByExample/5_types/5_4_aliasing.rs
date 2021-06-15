// nanosecond is a new name for u64
type NanoSecond = u64;
type Inch = u64;

// use an attribute to silence the warning
#[allow(non_camel_case_types)]
type u64_t = u64;

fn main() {
	// nanosecond = ince = u64_t = u64
	let nanoseconds: NanoSecond = 5 as u64_t;
	let inches: Inch = 2 as u64_t;

	// type aliases don't provide any extra type safety
	// aliases are not new types
	println!("{} nanoseconds + {} inches = {} unit?", 
		nanoseconds, inches, nanoseconds + inches)
}