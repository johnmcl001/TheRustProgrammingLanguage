fn main() {
    let _s1 = gives_ownership(); // Moves its return value into s1

    let s2 = String::from("hello"); // s2 comes into scope

    let _s3 = takes_and_gives_back(s2); // s2 moved into takes_and_gives_back
                                        // return value moved into s3
}

fn gives_ownership() -> String {
    let some_string = String::from("hello"); // String comes into scope
    some_string // some_string returned and moved out into calling function
}

// takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { // String comes into scope
    a_string //a_string is returned and moves out to the calling function
}
