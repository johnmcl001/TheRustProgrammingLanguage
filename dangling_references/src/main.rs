fn main() {
    // let reference_to_nothing = dangle();
    let _reference_to_s = no_dangle();
}

fn no_dangle() -> String {
    let s = String::from("hello");
    s
}

// fn dangle() -> &String { // returns a reference to a string
//     let s = String::from("hello"); // string comes into scope
//     &s // reference to s returned
// } // s goes out of scope, value in s dropped, memory goes away
