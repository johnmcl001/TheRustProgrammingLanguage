fn main() {
	// because of the annotation, the compiler knows that elem is u8
	let elem = 5u8;

	// create an empty vector
	let mut vec = Vec::new();
	// compiler doesn't know type of vec yet
	// just knows its a vector of something

	// insert elem into vec
	vec.push(elem);
	// compiler knows vec is type Vector<u8>
	// if this is commented out, need to give explicit type on vector declaration 

	println!("{:?}", vec);
}