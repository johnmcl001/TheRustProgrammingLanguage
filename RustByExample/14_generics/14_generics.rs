struct A;

// in defining the type Single, the first use of A is not preceded by <A>
// Therefore Single is a concrete type and A is defined as above
struct Single(A);

// Here, <T> preceds the first use of T so SingleGen is a generic type.
// because the type parameter T is generic it could be anything include A above
struct SingleGen<T>(T);

fn main() {
	// Single is concrete and explicitly takes an A
	let _s = Single(A);

	// Create a variable _char of type SingleGen<char>
	// give it the value SingleGen(a)
	// SingleGen has a type parameter explicitly specified
	let _char: SingleGen<char> = SingleGen('a');

	// SingleGen can also have a ypte parameter implicitly specified
	let _t = SingleGen(A);
	let _i32 = SingleGen(6);
	let _char = SingleGen('a');
}