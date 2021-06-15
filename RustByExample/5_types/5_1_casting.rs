// suppress all warnings from casts which overflow.
#![allow(overflowing_literals)]

fn main() {
	let decimal = 65.6321_f32;

	// Error: no implicit conversion
	// let integer: u8 = decimal;

	// Explicit conversion
	let integer = decimal as u8; // truncated
	let character = integer as char;

	// Error: limitations in conversion rules, floats can't be converted 
	// directly to characters
	// let character = decimal as char;

	println!("Casting: {} -> {} -> {}", decimal, integer, character);

	// when casting any value to an unsigned type, T, T::MAX + 1 is added or 
	// subtracted until the value fits into the new type

	// 1000 already fits in u16
	println!("1000 as a u16 is: {}", 1000 as u16);

	// 1000 - 256 - 256 - 256 = 232
	// Under the hood, the first 8 least significant bits are kept
	// The rest towwards the most significant bit get truncated
	println!("1000 as a u8 is: {}", 1000 as u8);
	// -1 + 256 = 255
	println!("-1 as a u8 is: {}", (-1i8) as u8);

	// For +ve numbers, this is the same as the modulus
	println!("1000 mod 256 is: {}", 1000 % 256);
	
	// When casting to a signed type, the bitwise result is the same as
	// first casting to the corresponding unsigned type. If the most 
	// significant bit of that value is 1 then the value is -ve.

	// Unless it already fits
	println!("128 as a i16 is: {}", 128 as i16);
	// 128 as u8 -> 128 whose 2's complement in eight bits is:
	println!("128 as a i8 is: {}", 128 as i8);

	// repeating the example above
	// 1000 as u8 -> 232 
	println!("1000 as a u8 is: {}", 1000 as u8);
	// the 2's complement of 232 is -24
	println!("232 as a u8 is: {}", 232 as i8);

	// since rust 1.45, as performs a saturating cast when casting from float to int
	// if the floating point value exceeds the upper bound or is less than the
	// lower bound, the returned value will be equal to the bound crossed.

	// 300.0 is 255
	println!("300.0 is {}", 300.0_f32 as u8);
	// -100.0 as u8 is 0
	println!("-100.0 is {}", -100.0_f32 as u8);
	// nan as u8 is 0
	println!("nan as u8 is {}", f32::NAN as u8);

	// This behavior incurs a small runtime cost and can be avoided using 
	// unsafe methods, the results may overflow and return unsound values
	// Use these methods sparingly
	unsafe {
		// 300.0 is 44
		println!("300.0 is {}", 300.0_f32.to_int_unchecked::<u8>());
		// -100.0 as u8 is 156
		println!("-100.0 as u8 is {}", (-100.0_f32).to_int_unchecked::<u8>());
		// nan as u8 is 0
		println!("nan as u8 is: {}", f32::NAN.to_int_unchecked::<u8>());
	}

}