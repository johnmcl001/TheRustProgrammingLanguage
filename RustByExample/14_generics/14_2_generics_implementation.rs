// struct S; // concrete type S
// struct GenericVal<T>(T); // generic type GenericVal

// // impl of GenericVal where types are explicit
// impl GenericVal<f32>{}
// impl GenericVal<S>{}

// // <T> must precede the type to remain generic
// impl<T> GenericVal<T>{}

struct Val {
	val: f64,
}

struct GenVal<T> {
	gen_val: T,
}

// impl of Val
impl Val {
	fn value(&self) -> &f64 {
		&self.val
	}
}

// impl of GenVal
impl<T> GenVal<T> {
	fn value(&self) -> &T {
		&self.gen_val
	}
}

fn main() {
	let x = Val{ val: 3.0 };
	let y = GenVal{ gen_val: 3i32 };

	println!("{}, {}", x.value(), y.value());
}
