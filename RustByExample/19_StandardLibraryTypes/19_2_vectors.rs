fn main() {
	// Iterators an be collected into vectors
	let collected_iterator: Vec<i32> = (0..10).collect();
	println!("Collected (0..10) into: {:?}", collected_iterator);
	
	// The vec! macro can be used to initialize a vector
	let mut xs = vec![1i32, 2, 3];
	println!("Initial vector: {:?}", xs);

	// Insert a new element at the end of a vector
	println!("Push 4 into vector");
	xs.push(4);
	println!("Vector: {:?}", xs);

	// Error: immutabe vectors can't grow
	// collected_iterator.push(0);

	// len yields the number of elements currently stored in the vector
	println!("Vector length: {}", xs.len());

	// indexing is done using square brackets
	println!("The second element: {}", xs[1]);

	// pop removes the last element from the vector and returns it
	println!("pop last element: {:?}", xs.pop());

	// Error: out of bounds panic
	// println!("Fourth element: {:?}", xs[3]);

	// vectors can be iterated over
	println!("Contents of xs");
	for x in xs.iter() {
		println!("> {}", x);
	}

	// vector can also be iterated over while the iteration count is enumerated
	for (i, x) in xs.iter().enumerate() {
		println!("The position of {} is {}", x, i);
	}

	// iter_mut lets mutable vectors be iterated over in a way that allows
	// modification
	for x in xs.iter_mut() {
		*x += 3;
	}
	println!("Updated vector: {:?}", xs);
}