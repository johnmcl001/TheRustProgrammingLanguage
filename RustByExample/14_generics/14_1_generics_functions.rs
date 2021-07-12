struct A;
struct S(A);
struct SGen<T>(T);

// The following functions all take ownership of the variable passed into them
// and immediately go out of scope, freeing the variable

// Define a function reg_fn that takes an argument _s of type S
// This has no <T> so this is not a generic function.
fn reg_fn(_s: S){}

// Define a function gen_spec_t that takes an argument _s of type SGen<T>
// it has been explicitly given the type parameter A, but because A has not
// been specified as a generic type parameter for gen_spec_t it is not generic
fn gen_spec_t(_s: SGen<A>){}

// Define a function gen_spec_i32 that takes an arg _s of type SGen<i32>
// it has been explicitly given the type parameter i32 which is a specific type
// because i32 is not a generic type, this function is also not generic
fn gen_spec_i32(_s: SGen<i32>){}

// Define function generic that takes an arg _s of type SGen<T>
// because SGen<T> is preceded by <T> this function is generic over T
fn generic<T>(_s: SGen<T>){}

fn main() {
	// using non-generic functions
	reg_fn(S(A));
	gen_spec_t(SGen(A));
	gen_spec_i32(SGen(6));

	// Explicitly specified type parameter char to generic()
	generic::<char>(SGen('a'));

	// Implicitly specified type parameter char to generic()
	generic(SGen('a'));
}